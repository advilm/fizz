import { useState } from 'react';
import { Popover, Avatar, ActionIcon, Button, Stack } from '@mantine/core';
import RegisterModal from './RegisterModal';
import LoginModal from './LoginModal';

export default function UserPopover() {
    const [opened, setOpened] = useState(false);
    const [registerOpened, setRegisterOpened] = useState(false);
    const [loginOpened, setLoginOpened] = useState(false);

    return (
        <>
            <RegisterModal opened={registerOpened} setOpened={setRegisterOpened}></RegisterModal>
            <LoginModal opened={loginOpened} setOpened={setLoginOpened}></LoginModal>
            <Popover
                opened={opened}
                onClose={() => setOpened(false)}
                width={150}
                position='bottom'
                placement='end'
                transition='fade'
                transitionDuration={100}
            >
                <Popover.Target>
                    <ActionIcon variant='filled' size={38} onClick={() => setOpened(!opened)}>
                        <Avatar></Avatar>
                    </ActionIcon>
                </Popover.Target>
                <Popover.Dropdown>
                    <Stack direction='column' grow>
                        <Button component='a' onClick={() => { setLoginOpened(true); setOpened(false); }}>Login</Button>
                        <Button component='a' onClick={() => { setRegisterOpened(true); setOpened(false); }}>Sign up</Button>
                    </Stack>
                </Popover.Dropdown>
            </Popover>
        </>
    );
}