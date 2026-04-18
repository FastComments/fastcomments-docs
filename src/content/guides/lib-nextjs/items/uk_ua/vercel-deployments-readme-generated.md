Якщо ви встановили FastComments через інтеграцію Vercel Marketplace, ваш tenant ID доступний як змінна середовища `FASTCOMMENTS_TENANT_ID`. Щоб прочитати його на клієнті, зробіть його доступним через `next.config.js` або додайте префікс `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```