# -*- coding: utf-8 -*-
"""SAP GUI 自动化模块执行器"""
import asyncio
import time
from typing import Any, Dict

from .base import ModuleExecutor, ExecutionContext, ModuleResult, register_executor
from .type_utils import to_int


def _resolve_session(context: ExecutionContext, session_var: str):
    session_info = context.get_variable(session_var)
    if not session_info or not isinstance(session_info, dict):
        raise RuntimeError(f'未找到 SAP 会话变量: {session_var}')
    session = session_info.get('session')
    if session is None:
        raise RuntimeError('SAP 会话对象无效，请重新登录')
    return session


@register_executor
class SapLoginExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_login'
    async def execute(self, config, context):
        conn_name = context.resolve_value(config.get('connName', ''))
        conn_string = context.resolve_value(config.get('connString', ''))
        username = context.resolve_value(config.get('username', ''))
        password = context.resolve_value(config.get('password', ''))
        mandt = context.resolve_value(config.get('mandt', '800'))
        language = context.resolve_value(config.get('language', 'ZH'))
        save_var = config.get('saveToVariable', 'sap_session')
        if not username: return ModuleResult(success=False, error='用户名不能为空')
        if not password: return ModuleResult(success=False, error='密码不能为空')
        if not conn_name and not conn_string: return ModuleResult(success=False, error='连接名称或连接字符串至少填写一个')
        try:
            result = await asyncio.get_running_loop().run_in_executor(None, self._login, conn_name, conn_string, username, password, mandt, language)
            if save_var: context.set_variable(save_var, result)
            return ModuleResult(success=True, message=f'SAP 登录成功，用户: {username}', data={'username': username, 'mandt': mandt})
        except Exception as e: return ModuleResult(success=False, error=f'SAP 登录失败: {str(e)}')
    def _login(self, conn_name, conn_string, username, password, mandt, language):
        import win32com.client, pythoncom, subprocess, psutil
        pythoncom.CoInitialize()
        try:
            if not any(p.name().lower() == 'saplogon.exe' for p in psutil.process_iter(['name'])):
                try:
                    import winreg
                    with winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE, r'SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\saplogon.exe') as k:
                        sap_path, _ = winreg.QueryValueEx(k, None)
                    subprocess.Popen(sap_path); time.sleep(5)
                except Exception: raise RuntimeError('SAP GUI 未启动，请手动打开 SAP Logon')
            gui = win32com.client.GetObject('SAPGUI')
            eng = gui.GetScriptingEngine
            eng.AllowSystemMessages = False; eng.HistoryEnabled = False
            if eng.Connections.Count > 0:
                for c in eng.Connections:
                    try: c.CloseSession('ses[0]')
                    except: pass
            conn = eng.OpenConnectionByConnectionString(conn_string) if conn_string else eng.OpenConnection(conn_name, True)
            if conn.DisabledByServer: raise RuntimeError('服务器已禁用 SAP GUI 脚本')
            ses = conn.Children(0)
            if ses.Busy: raise RuntimeError('SAP 会话正忙')
            ses.findById('wnd[0]/usr/txtRSYST-MANDT').text = str(mandt)
            ses.findById('wnd[0]/usr/txtRSYST-BNAME').text = str(username)
            ses.findById('wnd[0]/usr/pwdRSYST-BCODE').text = str(password)
            ses.findById('wnd[0]/usr/txtRSYST-LANGU').text = str(language)
            ses.findById('wnd[0]').sendVKey(0); time.sleep(1)
            try: ses.findById('wnd[0]').maximize()
            except: pass
            return {'session': ses, 'username': username, 'mandt': mandt, 'language': language}
        finally: pythoncom.CoUninitialize()


@register_executor
class SapLogoutExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_logout'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, self._logout, s)
            context.set_variable(sv, None)
            return ModuleResult(success=True, message='SAP 退出登录成功')
        except Exception as e: return ModuleResult(success=False, error=f'SAP 退出失败: {str(e)}')
    def _logout(self, s):
        s.findById('wnd[0]/tbar[0]/okcd').text = '/nex'
        s.findById('wnd[0]').sendVKey(0)


@register_executor
class SapRunTcodeExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_run_tcode'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        tcode = context.resolve_value(config.get('tcode', ''))
        if not tcode: return ModuleResult(success=False, error='事务码不能为空')
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, s.SendCommand, f'/n{tcode}')
            return ModuleResult(success=True, message=f'已执行事务码: {tcode}')
        except Exception as e: return ModuleResult(success=False, error=f'执行事务码失败: {str(e)}')


@register_executor
class SapSetFieldValueExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_set_field_value'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        eid = context.resolve_value(config.get('elementId', ''))
        val = context.resolve_value(config.get('value', ''))
        if not eid: return ModuleResult(success=False, error='元素ID不能为空')
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, self._set, s, eid, val)
            return ModuleResult(success=True, message=f'已设置 {eid} = {val}')
        except Exception as e: return ModuleResult(success=False, error=f'设置字段值失败: {str(e)}')
    def _set(self, s, eid, val): s.findById(eid).text = str(val)


@register_executor
class SapGetFieldValueExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_get_field_value'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        eid = context.resolve_value(config.get('elementId', ''))
        var = config.get('saveToVariable', 'sap_value')
        if not eid: return ModuleResult(success=False, error='元素ID不能为空')
        try:
            s = _resolve_session(context, sv)
            val = await asyncio.get_running_loop().run_in_executor(None, self._get, s, eid)
            if var: context.set_variable(var, val)
            return ModuleResult(success=True, message=f'获取到值: {val}', data={'value': val})
        except Exception as e: return ModuleResult(success=False, error=f'获取字段值失败: {str(e)}')
    def _get(self, s, eid):
        el = s.findById(eid)
        if hasattr(el, 'text'): return el.text
        if hasattr(el, 'Text'): return el.Text
        return ''


@register_executor
class SapClickButtonExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_click_button'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        eid = context.resolve_value(config.get('elementId', ''))
        if not eid: return ModuleResult(success=False, error='按鈕ID不能为空')
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, self._press, s, eid)
            return ModuleResult(success=True, message=f'已点击: {eid}')
        except Exception as e: return ModuleResult(success=False, error=f'点击失败: {str(e)}')
    def _press(self, s, eid): s.findById(eid).press()


@register_executor
class SapSendVKeyExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_send_vkey'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        vkey = to_int(config.get('vkey', 0), 0, context)
        wnd = to_int(config.get('windowIndex', 0), 0, context)
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, self._vkey, s, vkey, wnd)
            return ModuleResult(success=True, message=f'已发送虚拟键: {vkey}')
        except Exception as e: return ModuleResult(success=False, error=f'发送虚拟键失败: {str(e)}')
    def _vkey(self, s, vkey, wnd): s.findById(f'wnd[{wnd}]').sendVKey(vkey)


@register_executor
class SapGetStatusMessageExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_get_status_message'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        var = config.get('saveToVariable', 'sap_status_message')
        type_var = config.get('saveTypeVariable', '')
        wnd = to_int(config.get('windowIndex', 0), 0, context)
        try:
            s = _resolve_session(context, sv)
            r = await asyncio.get_running_loop().run_in_executor(None, self._get, s, wnd)
            if var: context.set_variable(var, r['message'])
            if type_var: context.set_variable(type_var, r['type'])
            return ModuleResult(success=True, message=f"状态消息: {r['message']}", data=r)
        except Exception as e: return ModuleResult(success=False, error=f'获取状态消息失败: {str(e)}')
    def _get(self, s, wnd):
        msg = s.findById(f'wnd[{wnd}]/sbar/pane[0]').text
        try: mt = s.findById(f'wnd[{wnd}]/sbar').messageType
        except: mt = ''
        return {'message': msg, 'type': mt}


@register_executor
class SapGetTitleExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_get_title'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        var = config.get('saveToVariable', 'sap_title')
        wnd = to_int(config.get('windowIndex', 0), 0, context)
        try:
            s = _resolve_session(context, sv)
            title = await asyncio.get_running_loop().run_in_executor(None, lambda: s.findById(f'wnd[{wnd}]').text)
            if var: context.set_variable(var, title)
            return ModuleResult(success=True, message=f'窗口标题: {title}', data={'title': title})
        except Exception as e: return ModuleResult(success=False, error=f'获取标题失败: {str(e)}')


@register_executor
class SapCloseWarningExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_close_warning'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, self._close, s)
            return ModuleResult(success=True, message='已关闭警告弹窗')
        except Exception as e: return ModuleResult(success=False, error=f'关闭警告弹窗失败: {str(e)}')
    def _close(self, s):
        cnt = len(s.children)
        for i in range(cnt - 1, 0, -1):
            try: s.findById(f'wnd[{i}]/tbar[0]/btn[0]').press()
            except: pass


@register_executor
class SapSetCheckboxExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_set_checkbox'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        eid = context.resolve_value(config.get('elementId', ''))
        checked = config.get('checked', True)
        if isinstance(checked, str): checked = checked.lower() == 'true'
        if not eid: return ModuleResult(success=False, error='元素ID不能为空')
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, self._set, s, eid, checked)
            return ModuleResult(success=True, message=f'复选框 {eid} = {checked}')
        except Exception as e: return ModuleResult(success=False, error=f'设置复选框失败: {str(e)}')
    def _set(self, s, eid, v): s.findById(eid).selected = v


@register_executor
class SapSelectComboBoxExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_select_combobox'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        eid = context.resolve_value(config.get('elementId', ''))
        key = context.resolve_value(config.get('key', ''))
        if not eid: return ModuleResult(success=False, error='元素ID不能为空')
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, self._sel, s, eid, key)
            return ModuleResult(success=True, message=f'下拉框 {eid} = {key}')
        except Exception as e: return ModuleResult(success=False, error=f'设置下拉框失败: {str(e)}')
    def _sel(self, s, eid, key): s.findById(eid).key = str(key)


@register_executor
class SapReadGridViewExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_read_gridview'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        eid = context.resolve_value(config.get('elementId', ''))
        var = config.get('saveToVariable', 'sap_table_data')
        if not eid: return ModuleResult(success=False, error='GridView ID不能为空')
        try:
            s = _resolve_session(context, sv)
            data = await asyncio.get_running_loop().run_in_executor(None, self._read, s, eid)
            if var: context.set_variable(var, data)
            return ModuleResult(success=True, message=f'读取 {len(data)} 行', data={'rows': len(data), 'data': data})
        except Exception as e: return ModuleResult(success=False, error=f'读取GridView失败: {str(e)}')
    def _read(self, s, eid):
        g = s.findById(eid)
        cols = list(g.ColumnOrder)
        rows = []
        for r in range(g.RowCount):
            rd = {}
            for c in cols:
                try: rd[c] = g.GetCellValue(r, c)
                except: rd[c] = ''
            rows.append(rd)
        return rows


@register_executor
class SapExportGridViewExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_export_gridview_excel'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        eid = context.resolve_value(config.get('elementId', ''))
        save_path = context.resolve_value(config.get('savePath', 'sap_export.xlsx'))
        if not eid: return ModuleResult(success=False, error='GridView ID不能为空')
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, self._export, s, eid, save_path)
            return ModuleResult(success=True, message=f'已导出到: {save_path}', data={'path': save_path})
        except Exception as e: return ModuleResult(success=False, error=f'导出Excel失败: {str(e)}')
    def _export(self, s, eid, save_path):
        try: import pandas as pd
        except ImportError: raise RuntimeError('请安装 pandas: pip install pandas openpyxl')
        g = s.findById(eid)
        cols = list(g.ColumnOrder)
        titles = {}
        for c in cols:
            try: titles[c] = g.GetDisplayedColumnTitle(c).strip() or c
            except: titles[c] = c
        rows = []
        for r in range(g.RowCount):
            rd = {}
            for c in cols:
                try: rd[titles[c]] = g.GetCellValue(r, c)
                except: rd[titles[c]] = ''
            rows.append(rd)
        pd.DataFrame(rows).to_excel(save_path, index=False)


@register_executor
class SapSetFocusExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_set_focus'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        eid = context.resolve_value(config.get('elementId', ''))
        if not eid: return ModuleResult(success=False, error='元素ID不能为空')
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, lambda: s.findById(eid).setFocus())
            return ModuleResult(success=True, message=f'已设置焦点: {eid}')
        except Exception as e: return ModuleResult(success=False, error=f'设置焦点失败: {str(e)}')


@register_executor
class SapMaximizeWindowExecutor(ModuleExecutor):
    @property
    def module_type(self): return 'sap_maximize_window'
    async def execute(self, config, context):
        sv = config.get('sessionVariable', 'sap_session')
        wnd = to_int(config.get('windowIndex', 0), 0, context)
        try:
            s = _resolve_session(context, sv)
            await asyncio.get_running_loop().run_in_executor(None, lambda: s.findById(f'wnd[{wnd}]').maximize())
            return ModuleResult(success=True, message='窗口已最大化')
        except Exception as e: return ModuleResult(success=False, error=f'最大化窗口失败: {str(e)}')
