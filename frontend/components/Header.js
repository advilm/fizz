import Link from 'next/link';
import { Header as MantineHeader, Text, ActionIcon, Avatar } from '@mantine/core';
import { BrandGithub } from 'tabler-icons-react';
import UserPopover from './UserPopover';

export default function Header() {
    return (
        <MantineHeader height={60} p="xs" sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', flex: 1 }}>         
            <Link href='/'>
                <a style={{ display: 'flex', alignItems: 'center', gap: 5, textDecoration: 'none', color: 'inherit' }}>
                    <Avatar src='/logo.svg'></Avatar>
                    <Text weight='bold' sx={{ letterSpacing: 1, fontSize: 25 }}>Fizz</Text>
                </a>
            </Link>
                    
            <div style={{ display: 'flex', gap: 10}}>
                <ActionIcon 
                    variant='filled' 
                    size={38} 
                    component='a' 
                    href='https://github.com/advilm/fizz' 
                    target='_blank' 
                    rel='noreferrer'
                >
                    <BrandGithub width='100%'/>
                </ActionIcon>
                <UserPopover/>
            </div>
        </MantineHeader>
    );
}
