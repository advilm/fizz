import React from 'react';
import { TextField } from '@mui/material';
import Box from './Box';
import { LoadingButton }from '@mui/lab';
import { useSnackbar } from 'notistack';
import Backdrop from './Backdrop';
import { motion } from 'framer-motion';
import { useFormik } from 'formik';
import * as yup from 'yup';

function TextInput({ id, label, formik, ...other }) {
	return (
		<TextField
			id={id}
			name={id}
			label={label}
			value={formik.values[id]}
			onChange={formik.handleChange}
			error={formik.touched[id] && Boolean(formik.errors[id])}
			helperText={formik.touched[id] && formik.errors[id] || ' '}
			variant='outlined'
			size='small'
			sx={{ pb: 0.5 }}
			{...other}
		/>
	);
}

export default function Login({ callback, login }) {
	const [loading, setLoading] = React.useState(false);
	
	const { enqueueSnackbar } = useSnackbar();
	
	const validationSchema = yup.object({
		email: login ? yup.string().email('Invalid email') : yup.string().email('Invalid email').required('Email is required'),
		username: yup.string().trim().required('Username is required')
			.matches(/\S{5}/, { message: 'Must be at least 5 characters' }),
		password: yup.string().trim().required('Password is required')
			.matches(/.{8}/, { message: 'Must be at least 8 characters' })
			.matches(/[A-Z]/, { message: 'Must have at least 1 uppercase letter' })
			.matches(/[a-z]/, { message: 'Must have at least 1 lowercase letter' })
			.matches(/\d/, { message: 'Must have at least 1 number' })
			.matches(/[@$!%*#?&]/, { message: 'Must have at least 1 symbol' })
			.matches(/^.{0,128}$/, { message: 'Must not exceed 128 characters' }),
	});
		
	const formik = useFormik({
		initialValues: {
			email: '',
			username: '',
			password: '',
		},
		validationSchema,
		onSubmit: async (values, actions) => {
			actions.resetForm();

			setLoading(true);
    
			const data = {
				username: values.username,
				password: values.password,
			};
    
			values.email && (data.email = values.email);
			const res = await global.fetch(
				process.env.NEXT_PUBLIC_API_BASE + '/users' + (data.email ? '/create' : '/login'),
				{
					body: JSON.stringify(data),
					method: 'POST',
					headers: { 'content-type': 'application/json' },
				},
			);
    
			setLoading(false);

			if (res.status == 201) {
				enqueueSnackbar(['success', 'User created succesfully']);
				callback(false);
			} else if (res.status == 409) {
				enqueueSnackbar(['error', 'Username or email taken']);
			} else {
				enqueueSnackbar(['error', 'An error occured']);
			}
		},
	});

	return (
		<Backdrop callback={callback}>
			<Box
				onClick={e => e.stopPropagation()}
				component={motion.div}
				position='fixed'
				className='fixed z-20 left-1/2 top-1/2 transform -translate-y-1/2 -translate-x-1/2 border-2 border-cyan rounded-md bg-black flex p-5'
				initial={{ opacity: 0 }}
				animate={{ opacity: 1 }}
				exit={{ opacity: 0, transition: { duration: .25 } }}
				transition={{ duration: 0.25 }}
			>
				{/* <button><X className='absolute right-3 top-3' onClick={() => callback(false)} size={24}></X></button> */}
				<form className='flex flex-col' onSubmit={formik.handleSubmit}>
					{login || <TextInput formik={formik} id='email' label='Email'/>}
					<TextInput formik={formik} id='username' label='Username' />
					<TextInput formik={formik} id='password' label='Password' type='password' />

					<LoadingButton type='submit' variant='outlined' size='medium' loading={loading}>{login ? 'Login' : 'Register'}</LoadingButton>
				</form>
			</Box>
		</Backdrop>
	);
}