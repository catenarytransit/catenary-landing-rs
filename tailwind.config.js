/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: "class",
    theme: {
        extend: {
            "sans": ["din-2014"]
        },
        colors: {
            border: 'hsl(var(--border))',
            input: 'hsl(var(--input))',
            ring: 'hsl(var(--ring))',
            background: 'hsl(var(--background))',
            foreground: 'hsl(var(--foreground))',
            primary: {
                DEFAULT: 'hsl(var(--primary))',
                foreground: 'hsl(var(--primary-foreground))',
            },
            secondary: {
                DEFAULT: 'hsl(var(--secondary))',
                foreground: 'hsl(var(--secondary-foreground))',
            },
            tertiary: {
                DEFAULT: 'hsl(var(--tertiary))',
                foreground: 'hsl(var(--tertiary-foreground))',
            },
            destructive: {
                DEFAULT: 'hsl(var(--destructive))',
                foreground: 'hsl(var(--destructive-foreground))',
            },
            muted: {
                DEFAULT: 'hsl(var(--muted))',
                foreground: 'hsl(var(--muted-foreground))',
            },
            accent: {
                DEFAULT: 'hsl(var(--accent))',
                foreground: 'hsl(var(--accent-foreground))',
            },
            popover: {
                DEFAULT: 'hsl(var(--popover))',
                foreground: 'hsl(var(--popover-foreground))',
            },
            card: {
                DEFAULT: 'hsl(var(--card))',
                foreground: 'hsl(var(--card-foreground))',
            },
            gray: {
                100: '#EBF1F5',
                200: '#D9E3EA',
                300: '#C5D2DC',
                400: '#9BA9B4',
                500: '#707D86',
                600: '#55595F',
                700: '#33363A',
                800: '#25282C',
                900: '#151719',
            },
            purple: {
                100: '#F4F4FF',
                200: '#E2E1FF',
                300: '#CBCCFF',
                400: '#ABABFF',
                500: '#8D8DFF',
                600: '#5D5DFF',
                700: '#4B4ACF',
                800: '#38379C',
                900: '#262668',
            },
        },
        spacing: {
            '9/16': '56.25%',
            '3/4': '75%',
            '1/1': '100%',
        },
        keyframes: {
            'accordion-down': {
                from: { height: 0 },
                to: { height: 'var(--radix-accordion-content-height)' },
            },
            'accordion-up': {
                from: { height: 'var(--radix-accordion-content-height)' },
                to: { height: 0 },
            },
        },
        animation: {
            'accordion-down': 'accordion-down 0.2s ease-out',
            'accordion-up': 'accordion-up 0.2s ease-out',
        },
        inset: {
            full: '100%',
        },
    },
    plugins: [],
}