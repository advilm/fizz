import { TextInput, PasswordInput, Button, Modal, LoadingOverlay } from '@mantine/core';
import { useForm } from '@mantine/form';
import { useState } from 'react';
import { useRouter } from 'next/router';
import { sleep } from '../lib/util';

export default function RegisterModal({ opened, setOpened}) {
    const form = useForm({
        initialValues: {
            username: '',
            password: '',
        },

        validate: {
            username: value =>
                value.length < 5 || value.length > 32 ? 'Username must be between 5 and 32 characters' : null,
            password: value => (value.length < 8 ? 'Password must be at least 8 characters' : null),
        },
    });

    const [loading, setLoading] = useState(false);

    const router = useRouter();

    return (
        <Modal
            opened={opened}
            onClose={() => setOpened(false)}
            title="Register"
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
                    const request = await fetch('http://localhost:3001/users/register',
                        {
                            method: 'POST',
                            headers: [['Content-Type', 'application/json']],
                            body: JSON.stringify(values)
                        });
                    await sleep(1000);
                    setLoading(false);

                    if (request.status === 409) {
                        form.setFieldError('username', 'Username already taken');
                    } else if (request.status === 201) {
                        const token = await request.text();
                        window.localStorage.setItem('token', token);
                        setOpened(false);
                        form.reset();
                        router.push('/dash');
                    }
                })}
                style={{ display: 'flex', flexDirection: 'column', gap: 10 }}
            >
                <TextInput
                    label='Username'
                    size='md'
                    sx={{ width: 300 }}
                    required
                    data-autofocus
                    {...form.getInputProps('username')}
                />

                <PasswordInput
                    label='Password'
                    size='md'
                    sx={{ width: 300 }}
                    required
                    {...form.getInputProps('password')}
                />

                <Button type="submit" mt='md'>Submit</Button>
            </form>
        </Modal>
    );
}