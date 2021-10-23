import React from 'react';
import Link from './Link';
import Login from './Login';
import { Button, Typography, AppBar } from '@mui/material';
import { AnimatePresence } from 'framer-motion';
import GitHubIcon from '@mui/icons-material/GitHub';
import Box from './Box';

export default function Layout({ children }) {
	const [login, setLogin] = React.useState(false);

	return (
		<Box flexDirection='column' min-height='100vh' position='relative'>
			<AnimatePresence exitBeforeEnter={true} initial={false}>
				{login && <Login callback={setLogin} login={true}/>}
			</AnimatePresence>
			<AppBar sx={{ display: 'flex', justifyContent: 'space-between', flexDirection: 'row', padding: 1.5, position: 'sticky' }}>
				<Link href='/' sx={{ display: 'flex', alignItems: 'center', textDecoration: 'none'}}>
					<img src='icon.svg' alt='logo' width='28' height='28'></img>
					<Typography marginLeft={1} fontWeight={700} fontSize='1.25rem' lineHeight='1.75rem'>Fizz</Typography>
				</Link>
				<Box>
					<Link href='https://github.com/advilm/fizz' target='_blank' rel='noreferrer' sx={{ marginRight: 1}}>
						<GitHubIcon fontSize='large'/>
					</Link>
					<Button variant='contained' onClick={() => setLogin(true)}>Login</Button>
				</Box>
			</AppBar>
			{children}
		</Box>
	);
}