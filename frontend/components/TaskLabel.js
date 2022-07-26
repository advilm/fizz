import { ActionIcon, Badge, Group, Text } from '@mantine/core';
import { secondsToTimeString } from '../lib/util';
import { Check, Pencil } from 'tabler-icons-react';
import React, { useState } from 'react';
import TaskModal from './TaskModal';

function TaskLabel({ task }) {
    const color = task.priority < 30 ? 'yellow'
        : task.priority < 70 ? 'orange'
            : 'red';
    const date = new Date(task.due * 1000);

    const [opened, setOpened] = useState(false);

    return (
        <>
            <TaskModal opened={opened} setOpened={setOpened} task={task}/>
            <Group position="apart" sx={{ marginRight: '12px' }}>
                <Text>{task.title}</Text>

                <Group>
                    <ActionIcon
                        variant='filled'
                        color='blue'
                        size={24}
                        onClick={e => e.stopPropagation()}
                    >
                        <Check/>
                    </ActionIcon>

                    <ActionIcon
                        variant='filled'
                        color='blue'
                        size={24}
                        onClick={e => {
                            e.stopPropagation();
                            setOpened(true);
                        }}
                    >
                        <Pencil/>
                    </ActionIcon>
                    <div style={{ width: '8em' }}>
                        <Badge fullWidth>
                            {
                                date.toLocaleDateString('en-US',
                                    {
                                        month: '2-digit',
                                        day: '2-digit',
                                        year: '2-digit',
                                        hour: '2-digit',
                                        minute: '2-digit'
                                    }).replace(',', '')
                            }
                        </Badge>
                    </div>
                    <div style={{ width: '2.9em' }}>
                        <Badge fullWidth>
                            {secondsToTimeString(task.time_estimate)}
                        </Badge>
                    </div>
                    <div style={{ width: '2.9em' }}>
                        <Badge color={color} fullWidth>{task.priority}</Badge>
                    </div>
                </Group>
            </Group>
        </>
    );
}

export default React.memo(TaskLabel);