# Anicap Frontend

This folder contains [Anicap](https://github.com/dcdaz/anicap) frontend

## Project setup
```
pnpm install
```

### Compiles and hot-reloads for development
```
pnpm run serve
```

### Compiles and minifies for production
```
pnpm run build
```

### Lints and fixes files
```
pnpm run lint
```

## Project Variables

Anicap uses env variables on the following files, `.env, .env.local, .env.production.local` but `*.local` are not versioned. If you want to run this project you need to create those files, with following vars

```
VUE_APP_BACKEND_URL=http://127.0.0.1:8085 // Your backend URL
```