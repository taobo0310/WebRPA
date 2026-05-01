"""全局浏览器管理器 - 通过 browser_engine 在主进程中共享 Playwright context"""
import asyncio
import json
import threading
import sys
from pathlib import Path
from typing import Optional

# 用户数据目录（供外部查询）
USER_DATA_DIR = Path(__file__).parent.parent.parent / "browser_data"
USER_DATA_DIR.mkdir(exist_ok=True)

# picker 脚本（保留，供 browser_engine 或独立进程使用）
_picker_active = False


def get_user_data_dir() -> str:
    return str(USER_DATA_DIR)


# =================== 状态查询 ===================

def is_browser_open() -> bool:
    """检查浏览器引擎是否已启动"""
    try:
        from app.services import browser_engine
        return browser_engine.is_open()
    except Exception:
        return False


def get_browser_proc():
    """兼容旧接口，返回 None（不再使用子进程）"""
    return None


def get_current_browser_config() -> dict:
    try:
        from app.services import browser_engine
        return browser_engine.get_config()
    except Exception:
        return {'type': 'msedge', 'executablePath': '', 'userDataDir': None}


# =================== 启动 / 停止 ===================

def start_browser(
    browser_type: str = 'msedge',
    executable_path: Optional[str] = None,
    user_data_dir: Optional[str] = None,
    fullscreen: bool = False,
    launch_args: Optional[str] = None,
) -> tuple[bool, str]:
    """启动浏览器（在主进程 event loop 中运行）"""
    try:
        from app.services import browser_engine
        loop = asyncio.get_event_loop()
        future = asyncio.run_coroutine_threadsafe(
            browser_engine.start(
                browser_type=browser_type,
                executable_path=executable_path,
                user_data_dir=user_data_dir,
                fullscreen=fullscreen,
                launch_args=launch_args,
            ),
            loop,
        )
        return future.result(timeout=60)
    except Exception as e:
        return False, str(e)


def stop_browser():
    """关闭浏览器"""
    global _picker_active
    _picker_active = False
    try:
        from app.services import browser_engine
        loop = asyncio.get_event_loop()
        future = asyncio.run_coroutine_threadsafe(browser_engine.stop(), loop)
        future.result(timeout=15)
    except Exception as e:
        print(f"[BrowserManager] stop_browser error: {e}")


# =================== 命令 ===================

def send_command(action: str, **kwargs) -> dict:
    """发送命令到浏览器引擎（异步转同步）"""
    if not is_browser_open():
        return {"success": False, "error": "浏览器未打开"}
    try:
        from app.services import browser_engine
        loop = asyncio.get_event_loop()

        async def _run():
            if action == 'navigate':
                return await browser_engine.navigate_to(kwargs.get('url', ''))
            elif action == 'find_page_by_url':
                return await browser_engine.find_page_by_url_async(kwargs.get('url', ''))
            elif action == 'switch_to_page':
                return await browser_engine.switch_to_page_async(kwargs.get('pageIndex', 0))
            elif action == 'start_picker':
                return await _start_picker_engine()
            elif action == 'stop_picker':
                return await _stop_picker_engine()
            elif action == 'get_selected':
                return await _get_picker_result('__elementPickerResult')
            elif action == 'get_similar':
                return await _get_picker_result('__elementPickerSimilar')
            elif action == 'quit':
                await browser_engine.stop()
                return {"success": True}
            else:
                return {"success": False, "error": f"未知命令: {action}"}

        future = asyncio.run_coroutine_threadsafe(_run(), loop)
        return future.result(timeout=30)
    except Exception as e:
        return {"success": False, "error": str(e)}


# =================== Picker 辅助 ===================

def _load_picker_script() -> str:
    # 直接内嵌精简版picker，避免依赖browser_process
    return """
(function() {
    if (window.__elementPickerActive) return;
    window.__elementPickerActive = true;
    var box = document.createElement('div');
    box.id = '__picker_box';
    box.style.cssText = 'position:fixed;pointer-events:none;border:3px solid #3b82f6;background:rgba(59,130,246,0.2);z-index:2147483647;border-radius:4px;display:none;';
    document.body.appendChild(box);
    var tip = document.createElement('div');
    tip.id = '__picker_tip';
    tip.style.cssText = 'position:fixed;top:10px;left:50%;transform:translateX(-50%);background:#1e40af;color:white;padding:10px 20px;border-radius:8px;font-size:14px;z-index:2147483647;font-family:sans-serif;';
    tip.textContent = 'Ctrl+点击选择元素';
    document.body.appendChild(tip);
    function getSelector(el) {
        if (el.id) return '#' + el.id;
        var path = [];
        while (el && el.nodeType === 1) {
            var s = el.tagName.toLowerCase();
            if (el.id) { s = '#' + el.id; path.unshift(s); break; }
            var sib = el.parentNode ? el.parentNode.children : [];
            var idx = Array.from(sib).indexOf(el) + 1;
            s += ':nth-child(' + idx + ')';
            path.unshift(s);
            el = el.parentNode;
        }
        return path.join(' > ');
    }
    document.addEventListener('mouseover', function(e) {
        var r = e.target.getBoundingClientRect();
        box.style.display = 'block';
        box.style.left = r.left + 'px'; box.style.top = r.top + 'px';
        box.style.width = r.width + 'px'; box.style.height = r.height + 'px';
    }, true);
    document.addEventListener('click', function(e) {
        if (e.ctrlKey) {
            e.preventDefault(); e.stopPropagation();
            window.__elementPickerResult = {
                selector: getSelector(e.target),
                tagName: e.target.tagName,
                text: e.target.innerText ? e.target.innerText.substring(0, 100) : '',
                attributes: Array.from(e.target.attributes).reduce(function(a,b){a[b.name]=b.value;return a;}, {})
            };
        }
    }, true);
})();
"""


PICKER_SCRIPT = _load_picker_script()


async def _start_picker_engine() -> dict:
    global _picker_active
    from app.services import browser_engine
    pg = browser_engine.get_page()
    if pg is None:
        return {"success": False, "error": "没有活跃页面"}
    try:
        await pg.evaluate(PICKER_SCRIPT)
        _picker_active = True
        return {"success": True, "data": {"message": "选择器已启动"}}
    except Exception as e:
        return {"success": False, "error": str(e)}


async def _stop_picker_engine() -> dict:
    global _picker_active
    from app.services import browser_engine
    _picker_active = False
    ctx = browser_engine.get_context()
    if ctx:
        for pg in ctx.pages:
            try:
                await pg.evaluate("""
                    () => {
                        ['__picker_tip','__picker_box'].forEach(id => {
                            var el = document.getElementById(id);
                            if (el) el.remove();
                        });
                        window.__elementPickerActive = false;
                    }
                """)
            except:
                pass
    return {"success": True, "data": {"message": "选择器已停止"}}


async def _get_picker_result(key: str) -> dict:
    from app.services import browser_engine
    pg = browser_engine.get_page()
    if pg is None:
        return {"success": True, "data": None}
    try:
        data = await pg.evaluate(f"""
            () => {{
                var r = window.{key};
                window.{key} = null;
                return r;
            }}
        """)
        return {"success": True, "data": data}
    except Exception as e:
        return {"success": False, "error": str(e)}


# =================== 便捷函数 ===================

def navigate(url: str) -> dict:
    return send_command("navigate", url=url)


def start_picker() -> dict:
    result = send_command("start_picker")
    return result


def stop_picker() -> dict:
    result = send_command("stop_picker")
    return result


def get_selected_element() -> dict:
    return send_command("get_selected")


def get_similar_elements() -> dict:
    return send_command("get_similar")


def is_picker_active() -> bool:
    return _picker_active


def find_page_by_url(url: str) -> dict:
    return send_command("find_page_by_url", url=url)


def switch_to_page(page_index: int) -> dict:
    return send_command("switch_to_page", pageIndex=page_index)


def ensure_browser_open(
    browser_type: str = 'msedge',
    executable_path: Optional[str] = None,
    fullscreen: bool = False,
) -> bool:
    from app.services import browser_engine
    if browser_engine.is_open():
        cfg = browser_engine.get_config()
        if (cfg.get('type') == browser_type and
                cfg.get('executablePath', '') == (executable_path or '') and
                True):
            return True
        # 配置变化，先停止
        stop_browser()
    success, _ = start_browser(browser_type, executable_path, None, fullscreen)
    return success
