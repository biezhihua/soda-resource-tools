import {api_basic_setting_get, api_basic_setting_update} from '@/services/soda/api';
import {PageContainer, ProForm, ProFormSelect, ProFormText} from '@ant-design/pro-components';
import {useIntl, useModel, useRequest} from '@umijs/max';
import {Button, Card, message, Spin, theme} from 'antd';
import React, {useState} from 'react';

const BasicSetting: React.FC = () => {
    const {token} = theme.useToken();
    const {initialState} = useModel('@@initialState');
    const {data, error, loading} = useRequest(() => {
        return api_basic_setting_get({
            skipErrorHandler: true,
        });
    });

    let type_options: { value: string; label: string; }[] = [];
    if (data?.system?.log?.output_type_options) {
        data?.system?.log?.output_type_options.forEach((value, index) => {
            if (value === "console") {
                type_options.push({
                    value: 'console',
                    label: '控制台',
                })
            }
        })
    }

    let level_options: { value: string; label: string; }[] = [];
    if (data?.system?.log?.output_level_options) {
        data?.system?.log?.output_level_options.forEach((value, index) => {
            switch (value) {
                case "info":
                    level_options.push({
                        value: 'info',
                        label: 'INFO',
                    });
                    break;

                case "debug":
                    level_options.push({
                        value: 'debug',
                        label: 'DEBUG',
                    });
                    break;
                case "error":
                    level_options.push({
                        value: 'error',
                        label: 'ERROR',
                    });
                    break;
            }
        })
    }

    const intl = useIntl();

    const [loadings, setLoadings] = useState<boolean[]>([]);

    const handleSubmit = async (values: API.UpdateBasicSettingParams) => {
        try {
            // 登录
            const result = await api_basic_setting_update({...values});

            if (result) {

                const msg = intl.formatMessage({
                    id: 'pages.setting.basic-setting.success',
                    defaultMessage: '保存成功!',
                });
                message.success(msg);
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
                            id: 'pages.setting.basic-setting.system',
                            defaultMessage: '系统',
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
                                await handleSubmit(values as API.UpdateBasicSettingParams);
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
                                                loading={loadings[0]}
                                                onClick={() => props.form?.submit?.()}>
                                                保存
                                            </Button>
                                        </div>,
                                    ];
                                },
                            }}
                        >
                            <ProForm.Group>
                                <ProFormSelect
                                    required={true}
                                    colProps={{md: 6}}
                                    initialValue={data?.system?.log?.output_type_value}
                                    options={type_options}
                                    width="md"
                                    name="log_output_type"
                                    label={'日志输出类型'}
                                />

                                <ProFormSelect
                                    required={true}
                                    colProps={{md: 6}}
                                    initialValue={data?.system?.log?.output_level_value}
                                    options={level_options}
                                    width="md"
                                    name="log_output_level"
                                    label={'日志输出级别'}
                                />

                            </ProForm.Group>

                            <ProForm.Group>
                                <ProFormText
                                    required={true}
                                    width="md"
                                    initialValue={data?.system?.web?.admin_username || ''}
                                    name="web_username"
                                    label="Web管理员用户名"
                                    tooltip=""
                                    colProps={{md: 6}}
                                    placeholder=""
                                />

                                <ProFormText.Password
                                    required={true}
                                    width="md"
                                    initialValue={data?.system?.web?.admin_password || ''}
                                    name="web_password"
                                    label="Web管理员密码"
                                    tooltip=""
                                    colProps={{md: 6}}
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

export default BasicSetting;
