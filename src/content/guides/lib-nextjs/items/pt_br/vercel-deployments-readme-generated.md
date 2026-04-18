Se você instalou o FastComments através da integração com o Vercel Marketplace, seu ID do tenant está disponível como a variável de ambiente `FASTCOMMENTS_TENANT_ID`. Para lê-lo no cliente, exponha-o através do `next.config.js` ou prefixe-o com `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```