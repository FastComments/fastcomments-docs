Ако сте инсталирали FastComments чрез интеграцията на Vercel Marketplace, идентификаторът на вашия tenant е наличен като променлива на средата `FASTCOMMENTS_TENANT_ID`. За да го прочетете в клиента, експонирайте го чрез `next.config.js` или го поставете с префикс `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```