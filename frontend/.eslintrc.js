module.exports = {
    root: true,
    env: {
        node: true,
        es2022: true,
    },
    extends: ['plugin:vue/vue3-essential'],
    rules: {
        'no-console': import.meta.env.NODE_ENV === 'production' ? 'warn' : 'off',
        'no-debugger': import.meta.env.NODE_ENV === 'production' ? 'warn' : 'off',
        'vue/multi-word-component-names': 'off'
    }
}
