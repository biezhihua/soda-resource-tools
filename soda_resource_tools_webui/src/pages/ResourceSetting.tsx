import {api_resource_setting_get, api_resource_setting_update} from '@/services/soda/api';
import {ActionType, PageContainer, ProForm, ProFormText} from '@ant-design/pro-components';
import {useIntl, useModel, useRequest} from '@umijs/max';
import {Button, Card, message, Spin} from 'antd';
import React, {useRef} from 'react';

const ResourceSetting: React.FC = () => {
  const {initialState} = useModel('@@initialState');
  const intl = useIntl();

  // 用于控制刷新UI
  const actionRef = useRef<ActionType>();

  const {data, loading} = useRequest(() => {
    return api_resource_setting_get({
      skipErrorHandler: true,
    });
  });

  const handleSubmit = async (values: API.ResourceSettingParams) => {
    try {
      // 登录
      const result = await api_resource_setting_update({...values});

      if (result) {

        const msg = intl.formatMessage({
          id: 'pages.setting.basic-setting.success',
          defaultMessage: '保存成功!',
        });
        message.success(msg);

        if (actionRef.current) {
          actionRef.current.reload();
        }

        return;
      }

    } catch (error) {
      const msg = intl.formatMessage({
        id: 'pages.setting.basic-setting.failure',
        defaultMessage: '保存失败，请重试！',
      });
      message.error(msg);
    }
  }

  return (
    <PageContainer>
      {loading && (
        <Spin style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center'
        }}/>
      )}

      {!loading && (
        <>
          <Card
            title={intl.formatMessage({
              id: 'pages.setting.resource-setting.rename',
              defaultMessage: '影视资源重命名规则',
            })}
            style={{
              borderRadius: 8,
            }}
            bodyStyle={{
              backgroundImage:
                initialState?.settings?.navTheme === 'realDark'
                  ? 'background-image: linear-gradient(75deg, #1A1B1F 0%, #191C1F 100%)'
                  : 'background-image: linear-gradient(75deg, #FBFDFF 0%, #F5F7FF 100%)',
            }}
          >

            <ProForm
              onFinish={async (values) => {
                await handleSubmit(values as API.ResourceSettingParams);
              }}
              grid={true}
              layout='vertical'
              onValuesChange={(changeValues) => console.log(changeValues)}
              rowProps={{
                gutter: [0, 0],
              }}
              submitter={{
                render: (props, doms) => {
                  return [
                    <div
                      style={{
                        float: 'right',
                      }}
                    >
                      <Button
                        type="primary"
                        onClick={() => props.form?.submit?.()}>
                        保存
                      </Button>
                    </div>,
                  ];
                },
              }}
            >

              <ProForm.Group>
                <ProFormText
                  initialValue={data?.rename_television || ''}
                  name="rename_television"
                  label="剧集规则"
                  tooltip="剧集分为三层目录：剧集文件夹/季文件夹/剧集名。可选配置有：。"
                  colProps={{md: 12}}
                  placeholder=""
                />

                <ProFormText
                  initialValue={data?.rename_film || ''}
                  name="rename_film"
                  label="电影规则"
                  tooltip="电影分为两层目录：电影文件夹/电影名。可选配置有：。"
                  colProps={{md: 12}}
                  placeholder=""
                />

              </ProForm.Group>
            </ProForm>
          </Card>
        </>
      )
      }
    </PageContainer>
  );
};

export default ResourceSetting;
