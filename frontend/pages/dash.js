import { AppShell, Text, Center, useMantineTheme } from '@mantine/core';
import { useRouter } from 'next/router';
import { useEffect } from 'react';
import Header from '../components/Header';

export default function Home() {
    const theme = useMantineTheme();
    const router = useRouter();

    useEffect(() => {
        const key = window.localStorage.getItem('token');
        if (!key) {
            router.push('/');
        } else {
            fetch('http://localhost:3001/tasks',
                {
                    method: 'GET',
                    headers: [['Authorization', `${key}`]]
                })
                .then(res => {
                    if (res.status === 401) {
                        router.push('/');
                    }
                });
        }
    });

    return (
        <AppShell
            sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}
            padding="md"
            header={<Header/>}
            styles={{
                body: {
                    flex: 1
                },
                main: {
                    backgroundColor: theme.colors.dark[8]
                }
            }}
        >
            <Center>
                <Text>
                test
                </Text>
            </Center>
        </AppShell>
    );
}
