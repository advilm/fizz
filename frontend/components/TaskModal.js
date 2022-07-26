import { TextInput, Button, Modal, LoadingOverlay, NumberInput, Box, NativeSelect } from '@mantine/core';
import { useForm } from '@mantine/form';
import { DatePicker, TimeInput } from '@mantine/dates';
import { useState } from 'react';
import RichTextEditor from './RichTextEditor';
import { intervalToSeconds, secondsToInterval, sleep } from '../lib/util';
import { atom, useSetRecoilState } from 'recoil';

const listState = atom({
    key: 'task_list',
    default: [],
});

export default function TaskModal({ opened, setOpened, task }) {
    const setList = useSetRecoilState(listState);
    const time = new Date();
    time.setHours(23);
    time.setMinutes(59);
    time.setMilliseconds(0);

    const recurringOptions = [
        { value: '0', label: 'Nope' },
        { value: '1', label: 'Daily' },
        { value: '2', label: 'Weekly' },
        { value: '3', label: 'Monthly' }
    ];

    const form = useForm({
        initialValues: {
            ...task,
            priority: task ? task.priority : 50,
            color: task ? task.color : 1667522,
            date: new Date(),
            recurring: '0',
            time_estimate: task ? secondsToInterval(task.time_estimate) : null,
            time,
        },
        validate: {
            title: value => value.length > 300 ? 'Title must less than 300 characters' : null,
            time_estimate: value => /^(?:\d+d)?(?:\d+h)?(?:\d+m)?(?:\d+s)?$/.test(value) ? null : 'Invalid'
        },
    });

    const [loading, setLoading] = useState(false);


    return (
        <Modal
            opened={opened}
            onClose={() => setOpened(false)}
            onClick={e => e.stopPropagation()}
            title="Add task"
            styles={{
                body: {
                    width: 'auto',
                },
                modal: {
                    width: 'auto'
                }
            }}
        >
            <LoadingOverlay visible={loading}/>
            <form
                onSubmit={form.onSubmit(async values => {
                    setLoading(true);

                    let due = new Date(values.date);
                    due.setHours(values.time.getHours());
                    due.setMinutes(values.time.getMinutes());
                    due.setSeconds(0);
                    due.setMilliseconds(0);

                    const body = {
                        title: values.title,
                        description: values.description,
                        priority: values.priority,
                        time_estimate: intervalToSeconds(values.time_estimate),
                        due: due.getTime() / 1000,
                        recurring: +values.recurring,
                        color: values.color,
                    };

                    if (!task) {
                        const { id } = await fetch('http://localhost:3001/tasks/add',
                            {
                                method: 'POST',
                                headers: [
                                    ['Content-Type', 'application/json'],
                                    ['Authorization', `${window.localStorage.getItem('token')}`]
                                ],
                                body: JSON.stringify(body)
                            }).then(res => res.json());

                        await sleep(1000);

                        body.id = id;
                        setList(list => [...list, body]);

                    } else {
                        body.completed = task.completed;
                        await fetch('http://localhost:3001/tasks/edit',
                            {
                                method: 'PUT',
                                headers: [
                                    ['Content-Type', 'application/json'],
                                    ['Authorization', `${window.localStorage.getItem('token')}`]
                                ],
                                body: JSON.stringify({
                                    ...body,
                                    id: task.id,
                                })
                            });

                        await sleep(1000);
                        setList(list => list.map(t => t.id === task.id ? body : t));
                    }

                    setLoading(false);
                    setOpened(false);
                    form.reset();
                })}
                style={{ display: 'flex', flexDirection: 'column', gap: '1em' }}
            >
                <TextInput
                    label='Title'
                    size='md'
                    required
                    data-autofocus
                    {...form.getInputProps('title')}
                />
                <Box sx={{ display: 'flex', flexDirection: 'row', gap: '1em' }}>
                    <NumberInput
                        label='Priority'
                        size='md'
                        required
                        min={0}
                        max={100}
                        {...form.getInputProps('priority')}
                    />
                    <TextInput
                        label='Time Estimate'
                        placeholder='1h'
                        size='md'
                        required
                        {...form.getInputProps('time_estimate')}
                    />
                    <DatePicker
                        label='Date due'
                        clearable={false}
                        size='md'
                        required
                        {...form.getInputProps('date')}
                    />
                    <NativeSelect
                        data={recurringOptions}
                        label="Recurring"
                        size="md"
                        required
                        {...form.getInputProps('recurring')}
                    />
                    <TimeInput
                        label='Time due'
                        size='md'
                        required
                        {...form.getInputProps('time')}
                    />
                </Box>
                <RichTextEditor
                    required
                    {...form.getInputProps('description')}
                />
                <Button type="submit" mt='md'>Submit</Button>
            </form>
        </Modal>
    );
}