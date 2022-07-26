import React from 'react';
import { useEffect } from 'react';
import { useRouter } from 'next/router';
import { atom, useRecoilState } from 'recoil';
import { Accordion } from '@mantine/core';
// import TaskLabel from './TaskLabel';
// import RichTextEditor from './RichTextEditor';
import TaskItem from './TaskItem';

const listState = atom({
    key: 'task_list',
    default: [],
});

export default function TaskList() {
    const router = useRouter();
    const [list, setList] = useRecoilState(listState);

    useEffect(() => {
        const key = window.localStorage.getItem('token');
        if (!key) {
            router.push('/');
        } else {
            fetch('http://localhost:3001/tasks/fetch',
                {
                    method: 'GET',
                    headers: [['Authorization', `${key}`]]
                })
                .then(async res => {
                    if (res.status === 401)
                        router.push('/');
                    else {
                        const json = await res.json();
                        setList(json);
                        console.log(json);
                    }
                });
        }
    }, [router, setList]);

    return (
        <Accordion sx={{ width: '100%' }} chevronPosition='left'>
            {list.map(task =>
                <TaskItem
                    key={task.id}
                    task={task}
                />
            )}
        </Accordion>
    );
}