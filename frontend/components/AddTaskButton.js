import { useState } from 'react';
import { ActionIcon } from '@mantine/core';
import { Plus } from 'tabler-icons-react';
import TaskModal from './TaskModal';

export default function AddTaskButton()
{
    const [opened, setOpened] = useState(false);

    return (
        <>
            <TaskModal opened={opened} setOpened={setOpened}/>
            <ActionIcon
                variant='filled'
                size={38}
                onClick={() => setOpened(true)}
            >
                <Plus/>
            </ActionIcon>
        </>);
}