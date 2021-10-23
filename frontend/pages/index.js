import React from 'react';
import { Button, Typography } from '@mui/material';
import Box from '../components/Box';
import Layout from '../components/Layout';
import Login from '../components/Login';
import { AnimatePresence } from 'framer-motion';

export default function Home() {
	const [login, setLogin] = React.useState(false);

	return (
		<Layout>
			<AnimatePresence exitBeforeEnter={true} initial={false}>
				{login && <Login callback={setLogin} login={false}/>}
			</AnimatePresence>
			<Box textAlign='center' flexDirection='column'>
				<Typography variant='h3' my='1rem' fontWeight='700' letterSpacing='-.025rem'>
					Open-source communications platform
				</Typography>
				<Button variant='contained' sx={{ fontSize: '1.125rem', py: '.625rem', lineHeight: '1.75rem' }} onClick={() => setLogin(true)}>
					Get Started
				</Button>
			</Box>
		</Layout>
	);
}
