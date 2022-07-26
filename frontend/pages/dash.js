import { AppShell, Center, useMantineTheme } from '@mantine/core';
import TaskList from '../components/TaskList';
import Header from '../components/Header';

export default function Home() {
    const theme = useMantineTheme();

    return (
        <AppShell
            sx={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}
            padding="md"
            header={<Header dash/>}
            styles={{
                body: {
                    flex: 1
                },
                main: {
                    backgroundColor: theme.colors.dark[8],
                    padding: 0
                }
            }}
        >
            <Center>
                <TaskList></TaskList>
            </Center>
        </AppShell>
    );
}