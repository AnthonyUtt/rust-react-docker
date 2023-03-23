module.exports = {
    exit: true,
    extension: ['js', 'jsx', 'ts', 'tsx'],
    recursive: true,
    require: [
        'utils/mocha-watch-cleanup-after-each.js', 
        'ts-node/register',
        'tsconfig-paths/register',
    ],
    spec: ['test/**/*.test.*'],
    'watch-files': ['src/**/*', 'test/**/*']
};
