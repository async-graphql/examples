const bsconfig = require('./bsconfig.json');
const withTM = require('next-transpile-modules')(
  ['bs-platform'].concat(bsconfig['bs-dependencies']),
);

const config = {
  crossOrigin: 'anonymous',
  webpack: (config, options) => {
    const rules = config.module.rules;

    // don't even ask my why
    config.node = {
      fs: 'empty',
    };

    // some react native library need this
    rules.push({
      test: /\.(gif|jpe?g|png|svg)$/,
      use: {
        loader: 'url-loader',
        options: {
          name: '[name].[ext]',
        },
      },
    });
    // .mjs before .js (fixing failing now.sh deploy)
    config.resolve.extensions = [
      '.wasm',
      '.mjs',
      '.web.js',
      '.web.jsx',
      '.ts',
      '.tsx',
      '.js',
      '.jsx',
      '.json',
      '.bs.js',
      '.gen.tsx',
    ];

    if (!options.isServer) {
      config.resolve.alias['@sentry/node'] = '@sentry/browser';
    }

    return config;
  },
  pageExtensions: ['jsx', 'js', 'web.js', 'web.jsx', 'ts', 'tsx', 'bs.js'],
  reactStrictMode: true,
  experimental: {
    reactMode: 'concurrent',
  },
};

module.exports = withTM(config);
