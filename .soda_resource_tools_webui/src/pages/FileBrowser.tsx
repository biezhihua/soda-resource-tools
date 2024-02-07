import {ActionType, PageContainer, ProColumns, ProField, ProTable} from "@ant-design/pro-components";
import React, {useEffect, useRef, useState} from "react";
import {api_actions, api_filebrowser_list} from "@/services/soda/api";
import {Breadcrumb, message} from "antd";
import {ItemType} from "antd/lib/breadcrumb/Breadcrumb";
import Icon, {ArrowUpOutlined, CopyOutlined, FileOutlined, FolderOutlined} from "@ant-design/icons";


const FileScrapeSvg = () => (
  <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
    <path
      d="M 6.4648438 2.9941406 A 0.50005 0.50005 0 0 0 6.0292969 3.3144531 L 2.5683594 9.2480469 A 0.50005 0.50005 0 1 0 3.4316406 9.7519531 L 6.5097656 4.4746094 L 7.8378906 6.6445312 L 4.7128906 12 L 1.5 12 A 0.50005 0.50005 0 0 0 1.0625 12.742188 L 6.0625 21.742188 A 0.50005 0.50005 0 0 0 6.5 22 L 17.347656 22 A 0.50005 0.50005 0 0 0 18.011719 21.607422 L 22.902344 12.804688 A 0.50005 0.50005 0 0 0 22.900391 12.191406 L 17.951172 3.2832031 L 17.949219 3.2773438 A 0.5011 0.5011 0 0 0 17.9375 3.2578125 A 0.50005 0.50005 0 0 0 17.923828 3.234375 A 0.5011 0.5011 0 0 0 17.910156 3.2148438 A 0.5011 0.5011 0 0 0 17.880859 3.1757812 A 0.5011 0.5011 0 0 0 17.845703 3.1386719 A 0.50005 0.50005 0 0 0 17.826172 3.1210938 A 0.5011 0.5011 0 0 0 17.808594 3.1074219 A 0.5011 0.5011 0 0 0 17.767578 3.078125 A 0.5011 0.5011 0 0 0 17.724609 3.0527344 A 0.5011 0.5011 0 0 0 17.677734 3.0332031 A 0.5011 0.5011 0 0 0 17.630859 3.0175781 A 0.5011 0.5011 0 0 0 17.585938 3.0058594 A 0.5011 0.5011 0 0 0 17.582031 3.0058594 A 0.50005 0.50005 0 0 0 17.578125 3.0058594 A 0.5011 0.5011 0 0 0 17.455078 3 L 6.5742188 3 A 0.50005 0.50005 0 0 0 6.4648438 2.9941406 z M 7.390625 4 L 16.615234 4 L 15.416016 6 L 8.6132812 6 L 7.390625 4 z M 17.484375 4.5019531 L 21.650391 12 L 19.308594 12 L 17.447266 8.2773438 A 0.50005 0.50005 0 1 0 16.552734 8.7226562 L 18.191406 12 L 16.289062 12 L 14.677734 9.1777344 L 17.484375 4.5019531 z M 9.2226562 7 L 14.816406 7 L 13.916016 8.5 L 10.138672 8.5 L 9.2226562 7 z M 8.4296875 7.6132812 L 9.3535156 9.125 L 7.7109375 12 L 5.8691406 12 L 8.4296875 7.6132812 z M 10.75 9.5 L 13.316406 9.5 L 12.044922 11.617188 L 10.75 9.5 z M 9.9492188 10.099609 L 11.111328 12 L 8.8632812 12 L 9.9492188 10.099609 z M 14.087891 10.162109 L 15.136719 12 L 12.984375 12 L 14.087891 10.162109 z M 2.3496094 13 L 4.7128906 13 L 8.0253906 18.675781 L 8.0292969 18.685547 A 0.50005 0.50005 0 0 0 8.5664062 19 L 15.386719 19 L 16.609375 21 L 6.7949219 21 L 2.3496094 13 z M 5.8691406 13 L 7.7324219 13 L 9.5664062 15.75 L 8.5058594 17.517578 L 5.8691406 13 z M 8.9335938 13 L 11.216797 13 L 10.134766 14.800781 L 8.9335938 13 z M 12.890625 13 L 15.066406 13 L 13.929688 14.703125 L 12.890625 13 z M 16.267578 13 L 18.128906 13 L 15.570312 17.386719 L 14.505859 15.642578 L 16.267578 13 z M 19.287109 13 L 21.650391 13 L 17.478516 20.507812 L 16.162109 18.355469 L 19.287109 13 z M 12.056641 13.546875 L 14.777344 18 L 9.3847656 18 L 12.056641 13.546875 z"></path>
  </svg>
);

const FileRecognizeSvg = () => (
  <svg data-v-6ed2d30f="" xmlns="http://www.w3.org/2000/svg"
       width="1em" height="1em" viewBox="0 0 24 24">
    <path fill="currentColor"
          d="M2 4c0-1.1.9-2 2-2h4v2H4v4H2V4m20 16c0 1.11-.89 2-2 2h-4v-2h4v-4h2v4M4 22a2 2 0 0 1-2-2v-4h2v4h4v2H4M20 2a2 2 0 0 1 2 2v4h-2V4h-4V2h4M9 7v2h2v8h2V9h2V7H9Z"></path>
  </svg>
);
const FileBrowser: React.FC = () => {

  // 用于控制是否设置了历史记录
  const [isPushState, setPushState] = useState<boolean>(false);

  // 用于控制刷新UI
  const actionRef = useRef<ActionType>();

  // 目标路径
  const [fileBrowserPath, setFileBrowserPath] = useState<string>('');

  // 存储自定义的历史数据
  const [historyStack] = useState([{
    title: '本地',
    record: {
      path: '',
      type: ''
    },
  }]);

  // 面包屑导航列表
  const [breadcrumbHistory, setBreadcrumbHistory] = useState<ItemType[]>([{
    title: '本地'
  }]);

  const doMapHistoryStack = () => {
    return historyStack.map((item) => {
      return {
        title: item.title,
        onClick: () => {
          let popItem = historyStack[historyStack.length - 1];
          while (popItem !== undefined && item.record.path !== popItem.record.path && historyStack.length > 1) {
            historyStack.pop();
            popItem = historyStack[historyStack.length - 1];
          }
          setFileBrowserPath(item.record.path);
          setBreadcrumbHistory(doMapHistoryStack());
          actionRef.current?.reload();
        }
      };
    });
  };

  const createIcon = (itemData: API.FileBrowserListItemResult) => {
    if (itemData.type === 'dir') {
      return <FolderOutlined style={{fontSize: '16px'}}/>;
    } else {
      return <FileOutlined style={{fontSize: '16px'}}/>;
    }
  };

  // 列样式
  const columns: ProColumns<API.FileBrowserListItemResult>[] = [
    {
      title: '名称',
      dataIndex: 'name',
      render: (_, itemData) => [
        <span key={itemData.name} style={{fontSize: '16px'}}>{createIcon(itemData)} {itemData.name}</span>,
      ],
    },
    {
      hideInTable: true,
      title: '类型',
      dataIndex: 'type',
      render: (_, itemData) => [
        <ProField
          key={'type'}
          text={itemData.type === 'dir' ? '文件夹' : '文件'}
          valueType="text"
          mode={'read'}
        />
      ],
    },
    {
      hideInTable: true,
      title: '修改时间',
      dataIndex: 'format_modify_time',
    },
    {
      title: '大小',
      width: 90,
      dataIndex: 'format_size',
      renderText: (val: string) => {
        if (val === '0 B') {
          return '';
        } else {
          return val;
        }
      }
    },
    {
      title: '操作',
      key: 'option',
      width: 90,
      valueType: 'option',
      render: (_, itemData) => [
        <CopyOutlined style={{fontSize: '16px'}} key={'copy_path'} title={'复制路径'} onClick={(e) => {
          e.stopPropagation();
          navigator.clipboard.writeText(itemData.path).then(() => {
            message.success('路径复制成功!');
          });
        }}/>,
        <Icon style={{fontSize: '16px'}} key={'file_recognize'} component={FileRecognizeSvg} title={'资源识别'}
              onClick={async (e) => {
                e.stopPropagation();
                console.log("file_recognize");
                try {
                  const result = await api_actions({
                    type: 'file_recognize',
                    file_path: itemData.path
                  });
                  message.success(JSON.stringify(result.data));
                } catch (e) {
                }
              }}/>
        ,
        <Icon style={{fontSize: '16px'}} key={'file_scrape'} component={FileScrapeSvg} title={'资源刮削'}
              onClick={async (e) => {
                e.stopPropagation();
                console.log("file_scrape");
                try {
                  const result = await api_actions({
                    type: 'file_scrape',
                    file_path: itemData.path
                  });
                  message.success(JSON.stringify(result.data));
                } catch (e) {
                }
              }}/>
      ],
    },
  ];

  const doBackFileBrowser = () => {
    historyStack.pop();
    let item = historyStack[historyStack.length - 1];
    if (item !== undefined) {
      setFileBrowserPath(item.record.path);
      setBreadcrumbHistory(doMapHistoryStack());
      actionRef.current?.reload();
    }
  }

  const createFileBrowserBackAction = () => {
    return <ArrowUpOutlined key={'file_browser_back'} onClick={() => {
      doBackFileBrowser();
    }}/>;
  };

  const createFileBrowserBreadcrumb = () => {
    return <Breadcrumb
      separator={'>'}
      items={breadcrumbHistory}
    />;
  };

  useEffect(() => {

    // 存储一个历史记录标记，阻止返回键
    if (historyStack.length !== 1 && !isPushState) {
      setPushState(true);
      window.history.pushState(null, document.title, window.location.href);
      // console.log("pushState");
    }
    // 如果存储过历史记录返回标记，那么将其消耗掉
    else if (historyStack.length === 1 && isPushState) {
      // console.log("back");
      setPushState(false);
      window.history.back();
    }

    return () => {
    };
  }, [fileBrowserPath]);

  useEffect(() => {
    const handlePopState = () => {

      // 处理鼠标返回键，如果有文件路由，那么存储一个历史记录标记阻止浏览器的返回行为，并使用自定义的文件浏览器返回。
      if (historyStack.length !== 1) {
        window.history.pushState(null, document.title, window.location.href);
        doBackFileBrowser();
        // console.log("handlePopState");
      }
    };

    // 添加popstate事件监听器
    window.addEventListener('popstate', handlePopState);

    const handleKeyDown = (e: WindowEventMap['keydown']) => {
      // 处理键盘返回键
      if (e.key === 'Backspace') {
        if (historyStack.length === 1) {
          e.preventDefault();
        } else {
          doBackFileBrowser();
        }
      }
    };

    // 添加键盘事件监听器
    window.addEventListener('keydown', handleKeyDown);

    return () => {
      // 在组件卸载时移除事件监听器，以避免内存泄漏
      window.removeEventListener('keydown', handleKeyDown);
      // 在组件卸载时移除popstate事件监听器，以避免内存泄漏
      window.removeEventListener('popstate', handlePopState);
    };
  }, [historyStack]);

  const doRequestFileBrowser = async () => {
    // 这里需要返回一个 Promise,在返回之前你可以进行数据转化
    // 如果需要转化参数可以在这里进行修改
    const result = await api_filebrowser_list(fileBrowserPath);
    return {
      data: result.data,
      success: true,
      total: result.data.length,
    };
  };

  return (<PageContainer>

    <ProTable<API.FileBrowserListItemResult, API.PageParams>
      actionRef={actionRef}
      size={'small'}
      pagination={{pageSize: 1024}}
      options={false}
      rowKey='path'
      headerTitle={'本地'}
      search={false}
      columns={columns}
      onRow={(record) => ({
        onClick: () => {
          if (record.type === 'dir') {
            historyStack.push({
              title: record.name,
              record: record
            })
            setBreadcrumbHistory(doMapHistoryStack());
            setFileBrowserPath(record.path);
            actionRef.current?.reload();
          }
        }, // 定义行点击事件处理函数
      })}
      request={async () => {
        return await doRequestFileBrowser();
      }}
      toolbar={{
        menu: {
          type: 'tab',
          activeKey: 'tab1',
          items: [
            {
              key: 'tab1',
              label: createFileBrowserBreadcrumb(),
            }
          ],
        },
        actions: [
          createFileBrowserBackAction()
        ],
      }}
    >
    </ProTable>
  </PageContainer>);
}

export default FileBrowser;
