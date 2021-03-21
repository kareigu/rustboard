module.exports = {
  purge: ['./templates/**/*.tera'],
  darkMode: false, // or 'media' or 'class'
  theme: {
    fontFamily: {
      'nunito': ['Nunito']
    },
    extend: {
      colors: {
        'main-dark-blue': '#0B0F19',
        'thread-accent': {
          'lt': '#FF7000',
          'md': '#F64C00',
          'dk': '#A93400'
        }
      },
      textColor: {
        'thread-link': '#234099'
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
