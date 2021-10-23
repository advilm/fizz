
import { createTheme } from '@mui/material/styles';

export default createTheme({
	typography: {
		fontFamily: [
			'"Segoe UI"',
			'Roboto',
			'-apple-system',
			'BlinkMacSystemFont',
			'"Helvetica Neue"',
			'Arial',
			'sans-serif',
			'"Apple Color Emoji"',
			'"Segoe UI Emoji"',
			'"Segoe UI Symbol"',
		].join(','),
		button: {
			textTransform: 'none',
		},
	},
	palette: {
		mode: 'dark',
	},
});