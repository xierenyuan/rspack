/**
 * @type {import('@rspack/cli').Configuration}
 */
module.exports = {
  entry: {
    main: './src/index.js'
  },
  builtins: {
    // html
    html: [
      {
        template: './index.html'
      }
    ]
  }
}