Ако сте инсталирали FastComments преко интеграције Vercel Marketplace-а, ваш tenant ID је доступан као променљива окружења `FASTCOMMENTS_TENANT_ID`. Да бисте га прочитали на клијенту, изложите га преко `next.config.js` или му додајте префикс `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```