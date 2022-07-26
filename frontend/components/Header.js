import Link from 'next/link';
import { Header as MantineHeader, Text, ActionIcon, Avatar, Group } from '@mantine/core';
import { BrandGithub } from 'tabler-icons-react';
import UserPopover from './UserPopover';
import AddTaskButton from './AddTaskButton';

export default function Header({ dash }) {
    return (
        <MantineHeader height={60} p="xs" sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', flex: 1 }}>
            <Link href='/'>
                <a style={{ display: 'flex', alignItems: 'center', gap: 5, textDecoration: 'none', color: 'inherit' }}>
                    <Avatar src='/logo.svg'></Avatar>
                    <Text weight='bold' sx={{ letterSpacing: 1, fontSize: 25 }}>Fizz</Text>
                </a>
            </Link>

            <Group spacing='xs'>
                {dash && <AddTaskButton/>}
                <ActionIcon
                    variant='filled'
                    size={38}
                    component='a'
                    href='https://github.com/advilm/fizz'
                    target='_blank'
                    rel='noreferrer'
                >
                    <BrandGithub/>
                </ActionIcon>
                <UserPopover/>
            </Group>
        </MantineHeader>
    );
}
