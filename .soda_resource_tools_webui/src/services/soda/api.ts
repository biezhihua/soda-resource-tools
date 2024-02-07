// @ts-ignore
/* eslint-disable */
import {request} from '@umijs/max';
import {getAuthorityAccessToken} from "@/utils/authority";

export async function api_basic_setting_get(options?: { [key: string]: any }) {
  return request<{
    data: API.BasicSettingResult
  }>('/api/basic/setting/get', {
    method: 'GET',
    headers: {
      'access_token': getAuthorityAccessToken()
    },
    ...(options || {})
  })
}

export async function api_basic_setting_update(body: API.UpdateBasicSettingParams, options?: { [key: string]: any }) {
  return request<{
    data: API.UpdateBasicSettingResult
  }>('/api/basic/setting/update', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'access_token': getAuthorityAccessToken()
    },
    data: body,
    ...(options || {}),
  });
}

/** 获取当前的用户 GET /api/currentUser */
export async function api_user_current_get(options?: { [key: string]: any }) {
  return request<{
    data: API.CurrentGetResult;
  }>('/api/user/current/get', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      'access_token': getAuthorityAccessToken()
    },
    ...(options || {}),
  });
}

export async function api_user_login_out(options?: { [key: string]: any }) {
  return request<Record<string, any>>('/api/user/login/out', {
    method: 'POST',
    headers: {
      'access_token': getAuthorityAccessToken()
    },
    ...(options || {}),
  });
}

export async function api_user_login_account(body: API.LoginParams, options?: { [key: string]: any }) {
  return request<{
    data: API.LoginResult;
  }>('/api/user/login/account', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    data: body,
    ...(options || {}),
  });
}


export async function api_resource_management_list(
  options?: { [key: string]: any }) {
  return request<API.ManagementRuleResult>('/api/resource/management/list', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    ...(options || {}),
  });
}


export async function api_resource_management_delete(id: number, options?: { [key: string]: any }) {
  return request(`/api/resource/management/delete/${id}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    ...(options || {}),
  });
}

export async function api_resource_management_create(params: API.ManagementRuleParams, options?: { [key: string]: any }) {
  return request('/api/resource/management/create', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    data: params,
    ...(options || {}),
  });
}

export async function api_resource_management_update(params: API.ManagementRuleParams, options?: { [key: string]: any }) {
  return request('/api/resource/management/update', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    data: params,
    ...(options || {}),
  });
}

export async function api_resource_management_action(id: number, options?: { [key: string]: any }) {
  return request(`/api/resource/management/action?id=${id}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    ...(options || {}),
  });
}


export async function api_filebrowser_list(target_path?: string, options?: { [key: string]: any }) {
  return request<API.FileBrowserListResult>(`/api/filebrowser/list?path=${target_path || ''}&sort=name_desc_and_type_desc`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    ...(options || {}),
  });
}


export async function api_resource_setting_get(options?: { [key: string]: any }) {
  return request<API.ResourceSettingResult>(`/api/resource/setting/get`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    ...(options || {}),
  });
}

export async function api_resource_setting_update(params: API.ResourceSettingParams, options?: { [key: string]: any }) {
  return request('/api/resource/setting/update', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    data: params,
    ...(options || {}),
  });
}

export async function api_actions(body: API.ActionsParams, options?: { [key: string]: any }) {
  console.log(`api_actions ${JSON.stringify(body)}`);
  return request<{
    data: API.ActionsResult
  }>('/api/actions', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'access_token': getAuthorityAccessToken()
    },
    data: body,
    ...(options || {}),
  });
}
