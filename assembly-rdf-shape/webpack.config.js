const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "tfgproject.js",
      webassemblyModuleFilename: "tfgproject.wasm"
    },
    module: {
      rules: [
        {
          test: /\.s[ac]ss$/i,
          use: [
            'style-loader',
            'css-loader',
            'sass-loader',
          ],
        },
      ],
      // rules: [
      //   {
      //     test: /\.css$/i,
      //     include: path.resolve(__dirname, 'src'),
      //     use: ['style-loader', 'css-loader', 'postcss-loader'],
      //   },
      // ],
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [
          { from: './src/yashe/yashe.js', to: distPath },  // Asegúrate de copiar yashe.js
          { from: './static', to: distPath },
          // { from: './node_modules/yashe/dist/yashe.min.css', to: distPath },
          // { from: './node_modules/yashe/dist/yashe.bundled.min', to: distPath },
          // { from: 'https://cdn.jsdelivr.net/npm/yashe@1.2.5/dist/yashe.bundled.min.js', to: distPath },
          //  { from: 'https://cdn.jsdelivr.net/npm/yashe@1.2.5/dist/yashe.min.css', to: distPath },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};
