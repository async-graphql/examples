const { colors } = require('tailwindcss/defaultTheme');

module.exports = {
  purge: ['./src/**/*.{js,jsx,ts,tsx,re}'],
  theme: {
    fontFamily: {
      sans: [
        '"Segoe UI"',
        'Roboto',
        '"Helvetica Neue"',
        'Arial',
        '"Noto Sans"',
        'sans-serif',
        '"Apple Color Emoji"',
        '"Segoe UI Emoji"',
        '"Segoe UI Symbol"',
        '"Noto Color Emoji"',
      ],
      serif: ['Georgia', 'Cambria', '"Times New Roman"', 'Times', 'serif'],
      mono: [
        'Menlo',
        'Monaco',
        'Consolas',
        '"Liberation Mono"',
        '"Courier New"',
        'monospace',
      ],
    },
    extend: {
      colors: {
        gray: {
          ...colors.gray,
          'mine-shaft': '#333333',
        },
      },
      screens: {
        'mobile-landscape': {
          raw: '(orientation: landscape) and ( max-height:550px )',
        },
      },
    },
  },
  variants: {},
  plugins: [require('@tailwindcss/ui')],
};
