module.exports = {
  content: ['./templates/**/*.tera'],
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
        'thread-link': '#FFFFFF'
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
