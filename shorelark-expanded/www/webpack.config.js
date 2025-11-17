const path = require('path');

module.exports = {
  mode: 'development',
  entry: './bootstrap.js',
  output: {
    path: __dirname,
    filename: 'bootstrap.js',
  },
  experiments: {
    asyncWebAssembly: true
  },
  resolve: {
    extensions: ['.js', '.wasm']
  },
  devServer: {
    static: {
      directory: __dirname,
    },
    open: true,
    compress: true,
    port: 8080
  }
};
