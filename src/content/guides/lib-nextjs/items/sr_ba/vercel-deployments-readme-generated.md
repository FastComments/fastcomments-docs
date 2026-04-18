Ако сте инсталирали FastComments преко интеграције Vercel Marketplace, ваш tenant ID је доступан као окружна варијабла `FASTCOMMENTS_TENANT_ID`. Да бисте га прочитали на клијенту, изложите га преко `next.config.js` или додајте префикс `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```