import {
  api_filebrowser_list,
  api_resource_management_create,
  api_resource_management_update
} from "@/services/soda/api";
import React, {useEffect, useState} from 'react';
import {message, TreeSelectProps} from 'antd';
import {DefaultOptionType} from "rc-select/es/Select";
import {ModalForm, ProForm, ProFormSelect, ProFormSwitch, ProFormTreeSelect} from "@ant-design/pro-components";

export type ManagementRuleModalProps = {
  modalType: string,
  initValue?: API.ManagementRuleParams | null;
  modalOpen: boolean;
  onOpenChange: (visible: boolean) => void;
  onFinish: () => Promise<void>;
};

const ManagementRuleModal: React.FC<ManagementRuleModalProps> = (props) => {

  const [pathTreeData, setPathTreeData] = useState<Omit<DefaultOptionType, 'label'>[]>([]);
  const [src, setSrc] = useState<string>();
  const [target, setTarget] = useState<string>();

  // create or update
  const isCreateType = () => {
    return props.modalType === 'create';
  }

  const onSrcChange = (newValue: string) => {
    setSrc(newValue);
  };

  const onTargetChange = (newValue: string) => {
    setTarget(newValue);
  };

  const convertFormItemToServerItem = (formItem: API.ManagementFormItem) => {
    let item: API.ManagementRuleParams = {
      id: props.initValue?.id || -1,
      src: formItem.src,
      target: formItem.target,
      content_type: formItem.content_type,
      period: formItem.period,
      mode: formItem.mode,
      status: 'running',
      monitor: 'running',
    };
    if (formItem.status) {
      item.status = 'running';
    } else {
      item.status = 'stop';
    }
    if (formItem.monitor) {
      item.monitor = 'running';
    } else {
      item.monitor = 'stop';
    }
    return item;
  }

  const handleFinish = async (formItem: API.ManagementFormItem) => {
    const serverItem = convertFormItemToServerItem(formItem);
    const hide = message.loading(isCreateType() ? '正在添加规则' : '正在更新规则');
    try {
      if (isCreateType()) {
        await api_resource_management_create(serverItem);
      } else {
        await api_resource_management_update(serverItem);
      }
      hide();
      message.success(isCreateType() ? '创建规则成功' : '更新规则成功');
      return true;
    } catch (error) {
      hide();
      message.error(isCreateType() ? '创建规则失败，请重试!' : '更新规则失败，请重试!');
      return false;
    }
  };

  const createTreeData = (target_path: string, response: API.FileBrowserListResult) => {
    const result: Omit<DefaultOptionType, 'label'>[] = [{}];
    if (response.data) {
      response.data.forEach((file_item) => {
        result.push({
          id: file_item.path,
          pId: target_path,
          value: file_item.path,
          title: file_item.path,
          isLeaf: !(file_item.type === 'dir')
        })
      })
    }
    return result;
  }

  const getPathTreeData = async (target_path?: string) => {
    const response = await api_filebrowser_list(target_path || '');
    return createTreeData(target_path || '', response);
  }

  const isExist = (target: Omit<DefaultOptionType, "label">, treeData: Omit<DefaultOptionType, "label">[]) => {
    if (target && target.id) {
      for (let i = 0; i < treeData.length; i++) {
        const src = treeData[i];
        if (src && src.id && target.id === src.id) {
          return true;
        }
      }
    }
    return false;
  }

  useEffect(() => {
    getPathTreeData().then(files => {
      files.forEach(file => {
        if (!isExist(file, pathTreeData)) {
          pathTreeData.push(file)
        }
      })
      setPathTreeData(pathTreeData.concat([]));
    });
    return () => {
    };
  }, []);

  const onLoadData: TreeSelectProps['loadData'] = ({id: parentId}) => {
    return new Promise((resolve) => {
      getPathTreeData(parentId).then(files => {
        files.forEach(file => {
          if (!isExist(file, pathTreeData)) {
            pathTreeData.push(file)
          }
        })
        setPathTreeData(pathTreeData.concat([]));
        resolve(undefined);
      });
    });
  }

  const getSrcInitValue = () => {
    return isCreateType() ? null : props.initValue?.src || '';
  }

  const getPathInitValue = () => {
    return isCreateType() ? null : props.initValue?.target || '';
  }

  const getContentTypeInitValue = () => {
    return isCreateType() ? null : props.initValue?.content_type || '';
  }

  const getModeInitValue = () => {
    return isCreateType() ? null : props.initValue?.mode || '';
  }

  const getPeriodInitValue = () => {
    return isCreateType() ? '24' : props.initValue?.period || '';
  }

  const getStatusInitValue = () => {
    return isCreateType() ? true : props.initValue?.status === 'running';
  }

  const getMonitorInitValue = () => {
    return isCreateType() ? true : props.initValue?.status === 'running';
  }

  return (<ModalForm
    title={isCreateType() ? '新建资源整理规则' : '更新资源整理规则'}
    open={props.modalOpen}
    onOpenChange={props.onOpenChange}
    onFinish={async (value) => {
      const success = await handleFinish(value as API.ManagementFormItem);
      if (success) {
        await props.onFinish();
      }
    }}
  >
    <ProForm.Group>
      <ProFormTreeSelect
        rules={[{required: true, message: '请选择资源源目录!'}]}
        required={true}
        name="src"
        label="源目录"
        tooltip="资源文件的原始目录。"
        width="md"
        initialValue={getSrcInitValue()}
        placeholder="请选择资源源目录"
        fieldProps={{
          treeDataSimpleMode: true,
          treeLine: true,
          treeData: pathTreeData,
          loadData: onLoadData,
          onChange: onSrcChange,
          value: src,
        }}
      />

      <ProFormTreeSelect
        rules={[{required: true, message: '请选择目的目录!'}]}
        required={true}
        name="target"
        label="目标目录"
        tooltip="资源整理的目标目录，资源文件经过soda处理后，将结果输出到该目录下。"
        width="md"
        initialValue={getPathInitValue()}
        placeholder="请选择目的目录"
        fieldProps={{
          treeDataSimpleMode: true,
          treeLine: true,
          treeData: pathTreeData,
          loadData: onLoadData,
          onChange: onTargetChange,
          value: target,
        }}
      />

    </ProForm.Group>

    <ProForm.Group>
      <ProFormSelect
        required={true}
        name="content_type"
        label="资源类型"
        tooltip="源目录的资源类型，例如：电影、电视剧、动漫、电子书、音乐等。准确的选择资源内容可以提高soda处理准确率。"
        width="md"
        initialValue={getContentTypeInitValue()}
        placeholder="请选择资源类型"
        rules={[{required: true, message: '请选择资源类型!'}]}
        request={async () => {
          return [
            {label: '影视', value: 'movie_and_tv'},
          ];
        }}
      />

      <ProFormTreeSelect
        required={true}
        name="mode"
        label="整理方式"
        tooltip="资源文件从源目录经soda处理到目标目录的方式，例如：硬链接、软连接、复制、移动等。默认为硬链接，目前仅支持硬链接。"
        width="md"
        initialValue={getModeInitValue()}
        placeholder="请选择整理方式"
        rules={[{required: true, message: '请选择整理方式!'}]}
        request={async () => {
          return [
            {label: '硬链接', value: 'hard_link'},
            {label: '软链接', value: 'soft_link'},
            {label: '复制', value: 'copy'},
            {label: '移动', value: 'move'},
          ];
        }}
      />
    </ProForm.Group>

    <ProForm.Group>

      <ProFormTreeSelect
        required={true}
        name="period"
        label="整理周期"
        tooltip="资源整理周期。默认为一周。"
        width="md"
        placeholder="请选择整理周期（小时）"
        initialValue={getPeriodInitValue()}
        rules={[{required: true, message: '请选择整理周期（小时）!'}]}
        request={async () => {
          return [
            {label: '一周', value: '168'},
            {label: '一月', value: '672'},
          ];
        }}
      />

      <ProFormSwitch
        tooltip="规则运行状态。"
        fieldProps={{
          checkedChildren: "运行中",
          unCheckedChildren: "停止",
        }}
        width="md"
        initialValue={getStatusInitValue()}
        name="status"
        label="运行状态"/>

      <ProFormSwitch
        tooltip="目录监控状态。"
        fieldProps={{
          checkedChildren: "运行中",
          unCheckedChildren: "停止",
        }}
        width="md"
        initialValue={getMonitorInitValue()}
        name="monitor"
        label="监控状态"/>
    </ProForm.Group>
  </ModalForm>);
};

export default ManagementRuleModal;
