Ако сте инсталирали FastComments преко интеграције Vercel Marketplace-а, ваш tenant ID је доступан као променљива окружења `FASTCOMMENTS_TENANT_ID`. Да бисте га читали на клијенту, изложите га кроз `next.config.js` или додајте префикс `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```