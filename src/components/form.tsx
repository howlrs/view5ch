import { invoke } from "@tauri-apps/api/core";
import { Button, Input, message, Segmented, Space, Spin, Tabs, TabsProps } from "antd";
import { useState } from "react";

interface jsonMap {
    [key: string]: any;
}

interface TargetItem {
    title: string;
    url?: string;
    option?: string;
};

const SegmentedOptions = ['リスト抽出', '対象タグ抽出', 'データ保存'];

const FormComponent = () => {
    const [loading, setLoading] = useState<boolean>(false);
    const [selectedSegment, setSelectedSegment] = useState<string>(SegmentedOptions[0]);

    const [target, setTarget] = useState<string>('');
    const [targetTag, setTargetTag] = useState<string>('li');

    const [items, setItems] = useState<TargetItem[]>([]);
    const [tags, setTags] = useState<jsonMap>([]);

    const reqestSwitcher = async () => {
        setLoading(true);
        try {
            const param = selectedSegment;
            switch (param) {
                case SegmentedOptions[0]:
                    getList();
                    break;
                case SegmentedOptions[1]:
                    getComments();
                    break;
                case SegmentedOptions[2]:
                    saveValue();
                    break;
                default:
                    return '';
            }
        } catch (e) {
            console.log(e);
        } finally {
            setLoading(false);
        }
    };

    const getList = async () => {
        try {
            const res: any = await invoke('get_list', { target_url: target });
            setItems(res.data as TargetItem[]);
            setTags(res.option as jsonMap[]);

            message.open({
                type: 'success',
                duration: 3,
                content: res.message,
            });
        } catch (e: any) {
            message.open({
                type: 'error',
                duration: 3,
                content: e,
            });
        }
    };


    const getComments = async () => {
        try {
            const res: any = await invoke('get_comments', { target_url: target, tag: targetTag });
            setItems(res.data as TargetItem[]);
            message.open({
                type: 'success',
                duration: 3,
                content: res.message,
            });
        } catch (e: any) {
            message.open({
                type: 'error',
                duration: 3,
                content: e,
            });
        }
    };

    const saveValue = async () => {
        try {
            const targetValue = items;
            if (!targetValue) {
                throw new Error('targetValue is empty');
            }

            const res: any = await invoke('save_value', { value: targetValue });
            message.open({
                type: 'success',
                duration: 3,
                content: res.message,
            });
        } catch (e: any) {
            message.open({
                type: 'error',
                duration: 3,
                content: e.message,
            });
        }
    };

    const tabItems: TabsProps['items'] = [
        {
            key: '1',
            label: 'リスト',
            children: items && items.length > 0 ? (
                <>
                    <h2>リスト</h2>
                    <ul>
                        {items.map((item, index) => {
                            const content = `${item.title} ${item.url ? `: <a href="${item.url}">${item.url}</a>` : ''} ${item.option ? ':' + item.option : ''}`;
                            return <li key={index} dangerouslySetInnerHTML={{ __html: content }}></li>;
                        })}
                    </ul>
                </>
            ) : null,
        },
        {
            key: '2',
            label: 'タグ',
            children: tags && Object.keys(tags).length > 0 ? (
                <>
                    <h2>頻出タグ</h2>
                    <ul>
                        {
                            // jsonMap view
                            // sorted by value (descending order)
                            Object.entries(tags)
                                .sort(([, a], [, b]) => Number(b) - Number(a))
                                .map(([key, value], index) => {
                                    return <li key={index}>{key} : {value}</li>;
                                })
                        }
                    </ul>
                </>
            ) : null,
        },
    ];


    return (
        <Spin spinning={loading}>
            <Space direction="vertical" size="large" style={{ width: '80wv', padding: '2rem' }}>
                <Segmented options={SegmentedOptions} onChange={
                    (e) => {
                        setSelectedSegment(e);
                    }
                }></Segmented>


                {
                    selectedSegment != SegmentedOptions[2] && (
                        <Input placeholder="対象URL" onChange={
                            (e) => {
                                setTarget(e.target.value);
                            }
                        } />
                    ) || null
                }

                {
                    selectedSegment === SegmentedOptions[1] && (
                        <Input placeholder="対象タグ" onChange={
                            (e) => {
                                setTargetTag(e.target.value);
                            }
                        } />
                    ) || null
                }


                <Button type="primary" onClick={reqestSwitcher}>
                    実行
                </Button>
            </Space>

            <Tabs centered defaultActiveKey="1" items={tabItems} />
        </Spin >
    );
};

export default FormComponent;