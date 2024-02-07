import type {ActionType, ProColumns} from '@ant-design/pro-components';
import {PageContainer, ProTable, TableDropdown} from "@ant-design/pro-components";
import {
  api_resource_management_action,
  api_resource_management_delete,
  api_resource_management_list
} from "@/services/soda/api";
import React, {useRef, useState} from 'react';
import {Button, message} from 'antd';
import ManagementRuleModal from "@/components/ManagementRuleModal";

const ManagementRule: React.FC = () => {

  // 用于控制新建Modal是否显示
  const [modalCreate, setModalCreate] = useState<boolean>(false);
  const [modalUpdate, setModalUpdate] = useState<boolean>(false);
  const [modalModalUpdateItem, setModalUpdateItem] = useState<API.ManagementRuleParams>();
  const [selectedRows, setSelectedRows] = useState<API.ManagementRuleParams[]>([]);

  // 用于控制刷新UI
  const actionRef = useRef<ActionType>();

  const columns: ProColumns<API.ManagementRuleParams>[] = [
    {
      title: '源目录',
      dataIndex: 'src',
      tip: '资源文件的原始目录，一般是资源下载目录。'
    },

    {
      title: '目标目录',
      dataIndex: 'target',
      tip: '资源整理的目标目录，资源文件经过soda处理后，将结果输出到该目录下。'
    },
    {
      title: '内容类型',
      dataIndex: 'content_type',
      tip: '源目录的资源类型，例如：电影、电视剧、动漫、电子书、音乐等。准确的选择资源内容可以提高处理准确率。',
      valueEnum: {
        movie_and_tv: {text: '影视'},
      }
    },
    {
      title: '整理方式',
      dataIndex: 'mode',
      tip: '资源文件从源目录经soda处理到目标目录的方式，例如：硬链接、软连接、复制、移动等。默认为硬链接，目前仅支持硬链接。',
      valueEnum: {
        hard_link: {text: '硬链接'},
      },
    },
    {
      title: '整理周期',
      dataIndex: 'period',
      tip: '资源整理周期。默认为一周。',
      valueEnum: {
        168: {text: '一周'},
        672: {text: '一月'},
        // 48: {text: '48小时'},
        // 72: {text: '72小时'},
      }
    },
    {
      title: '运行状态',
      dataIndex: 'status',
      initialValue: 'running',
      tip: '规则运行状态。',
      valueEnum: {
        running: {text: '运行中', status: 'Processing'},
        stop: {text: '停止', status: 'Error'},
      },
    },
    {
      title: '监控状态',
      dataIndex: 'monitor',
      initialValue: 'running',
      tip: '监控运行状态。',
      valueEnum: {
        running: {text: '运行中', status: 'Processing'},
        stop: {text: '停止', status: 'Error'},
      },
    },
    {
      title: '操作',
      key: 'option',
      valueType: 'option',
      render: (_, itemData) => [
        <TableDropdown
          onSelect={async (key) => {
            if ("delete" === key) {
              await api_resource_management_delete(itemData.id);
              actionRef.current?.reloadAndRest?.();
            } else if ("edit" === key) {
              setModalUpdateItem(itemData);
              setModalUpdate(true);
            } else if ("management" === key) {
              api_resource_management_action(itemData.id).then();
            }
          }}
          key="actionGroup"
          menus={[
            {key: 'management', name: '整理'},
            {key: 'edit', name: '编辑'},
            {key: 'delete', name: '删除'},
          ]}
        />,
      ],
    },
  ];

  return (
    <PageContainer>
      <ProTable<API.ManagementRuleParams, API.PageParams>
        actionRef={actionRef}
        rowKey="id"
        request={api_resource_management_list}
        options={false}
        rowSelection={{
          onSelect: ((record, selected, selectedRows) => {
            setSelectedRows(selectedRows);
          }),
          alwaysShowAlert: false
        }}
        headerTitle={''}
        pagination={{
          showQuickJumper: true,
        }}
        columns={columns}
        search={false}
        dateFormatter="string"
        toolBarRender={() => [
          <Button type="primary" key="primary" onClick={() => {
            setModalCreate(true);
          }
          }>
            新建
          </Button>,
          <Button type="primary" key="primary" onClick={() => {
            if (selectedRows.length === 0) {
              message.error('请至少选择一个资源整理规则');
            } else {
              selectedRows.forEach((value) => {
                api_resource_management_delete(value.id).then();
              })
            }
          }
          }>
            删除
          </Button>,
          <Button type="primary" key="primary" onClick={() => {
            if (selectedRows.length === 0) {
              message.error('请至少选择一个资源整理规则');
            } else {
              selectedRows.forEach((value) => {
                api_resource_management_action(value.id).then();
              })
            }
          }
          }>
            整理
          </Button>,
        ]}
      />

      <ManagementRuleModal modalType={'update'} initValue={modalModalUpdateItem}
                           modalOpen={modalUpdate}
                           onOpenChange={setModalUpdate}
                           onFinish={async () => {
                             setModalUpdate(false)
                             if (actionRef.current) {
                               actionRef.current.reload();
                             }
                           }}/>


      <ManagementRuleModal modalType={'create'} initValue={null}
                           modalOpen={modalCreate}
                           onOpenChange={setModalCreate}
                           onFinish={async () => {
                             setModalCreate(false)
                             if (actionRef.current) {
                               actionRef.current.reload();
                             }
                           }}/>

    </PageContainer>
  );
};

export default ManagementRule
