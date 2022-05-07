import { TextInput, PasswordInput, Button, Modal, LoadingOverlay } from '@mantine/core';
import { useForm } from '@mantine/form';
import { useState } from 'react';
import { useRouter } from 'next/router';

export default function RegisterModal({ opened, setOpened}) {
    const form = useForm({
        initialValues: {
            email: '',
            password: '',
        },

        validate: {
            email: (value) => (/^\S+@\S+$/.test(value) ? null : 'Invalid email'),
            password: (value) => (value.length < 8 ? 'Password must be at least 8 characters' : null),
        },
    });

    const [loading, setLoading] = useState(false);

    const router = useRouter();
    const sendToDash = () => {
        router.push('/dash');
    };

    return (
        <Modal
            opened={opened}
            onClose={() => setOpened(false)}
            title="Login"
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
                    const request = await fetch('http://localhost:3001/users/login',
                        {
                            method: 'POST',
                            headers: [['Content-Type', 'application/json']],
                            body: JSON.stringify(values)
                        });
                    await sleep(1000);
                    setLoading(false);

                    if (request.status == 401) {
                        form.setFieldError('password', 'Password incorrect');
                    } else if (request.status == 404) {
                        form.setFieldError('email', 'Email not found');
                    } else if (request.status == 200) {
                        const token = await request.text();
                        window.localStorage.setItem('token', token);
                        setOpened(false);
                        sendToDash();
                    }
                })}
                style={{ display: 'flex', flexDirection: 'column', gap: 10 }}
            >
                <TextInput
                    label='Email'
                    size='md'
                    sx={{ width: 300 }}
                    required
                    data-autofocus
                    {...form.getInputProps('email')}
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

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}