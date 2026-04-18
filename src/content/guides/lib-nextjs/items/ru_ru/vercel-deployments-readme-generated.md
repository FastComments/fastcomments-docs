Если вы установили FastComments через интеграцию Vercel Marketplace, ваш tenant ID доступен как переменная окружения `FASTCOMMENTS_TENANT_ID`. Чтобы прочитать его на клиенте, откройте доступ через `next.config.js` или добавьте префикс `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```