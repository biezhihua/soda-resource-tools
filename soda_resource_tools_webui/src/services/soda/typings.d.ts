// @ts-ignore
/* eslint-disable */

declare namespace API {

  type ErrorResponse = {
    /** 业务约定的错误码 */
    errorCode: string;
    /** 业务上的错误信息 */
    errorMessage?: string;
    /** 业务上的请求是否成功 */
    success?: boolean;
  };

  type PageParams = {
    current?: number;
    pageSize?: number;
  };

  type LoginParams = {
    username?: string;
    password?: string;
    auto_login?: boolean;
  };

  type LoginResult = {
    status?: string;
    access_token?: string;
  };

  type UpdateBasicSettingParams = {
    log_output_level?: string;
    log_output_type?: string;
    web_password?: string;
    web_username?: string;
  }

  type ActionsParams = {
    type: string;
    file_path?: string;
  }

  type ActionsResult = {
    data?: {
      metadata?: any
    }
  };

  type BasicSettingResult = {
    system?: {
      log?: {
        output_type_options?: string[];
        output_type_value?: string;
        output_level_options?: string[];
        output_level_value?: string;
      };
      web?: {
        admin_username?: string;
        admin_password?: string;
        admin_nickname?: string;
      };
    }
  }

  type CurrentGetResult = {
    name?: string;
    permission?: string;
    avatar?: string;
  };

  /**
   * 资源整理列表的结果数据
   */
  type ManagementRuleResult = {
    data?: ManagementRuleParams[];
    total?: number;
    success?: boolean;
  };

  /**
   * 资源整理列表的数据
   */
  type ManagementRuleParams = {
    id: number;
    src: string;
    target: string;
    content_type?: string;
    mode: string;
    period: number;
    status: string;
    monitor: string;
  };

  /**
   * 资源整理的表单数据
   */
  type ManagementFormItem = {
    id: number;
    src: string;
    target: string;
    content_type?: string;
    mode: string;
    period: number;
    status: boolean;
    monitor: boolean;
  };

  type UpdateBasicSettingResult = {};

  type FileBrowserListResult = {
    data: FileBrowserListItemResult[];
  };

  type FileBrowserListItemResult = {
    type: string,
    path: string,
    name: string,
    extension?: string,
    size?: number,
    format_size?: string,
    modify_name?: number
  };

  type ResourceSettingResult = {
    data: {
      rename_film?: string;
      rename_television?: string;
    };
  };

  type ResourceSettingParams = {
    rename_film?: string;
    rename_television?: string;
  }

}
