Если вы установили FastComments через интеграцию Vercel Marketplace, ваш tenant ID доступен как переменная окружения `FASTCOMMENTS_TENANT_ID`. Чтобы прочитать его на клиенте, сделайте его доступным через `next.config.js` или добавив префикс `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```