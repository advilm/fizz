import React from 'react';
import { Accordion } from '@mantine/core';
import TaskLabel from './TaskLabel';
import RichTextEditor from './RichTextEditor';

function TaskItem({ task }) {

    return (
        <Accordion.Item
            value={'' + task.id}
        >
            <Accordion.Control>
                <TaskLabel task={task}/>
            </Accordion.Control>
            <Accordion.Panel>
                <RichTextEditor
                    value={task.description}
                    readOnly
                    sx={{ p: { marginBottom: 0 } }} />
            </Accordion.Panel>
        </Accordion.Item>
    );
}

export default React.memo(TaskItem);