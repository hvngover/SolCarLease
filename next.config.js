const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const path = require('path');

module.exports = {
    webpack: (config, options) => {
        config.experiments = {
            asyncWebAssembly: true,
            layers: true,
        };
        config.plugins.push(
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, './solcarlease'),
            })
        );
        return config;
    },
};