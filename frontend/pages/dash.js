import { AppShell, Text, Center, useMantineTheme } from '@mantine/core';
import Header from '../components/Header';

export default function Home() {
    const theme = useMantineTheme();

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
