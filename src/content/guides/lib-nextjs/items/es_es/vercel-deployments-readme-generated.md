---
Si ha instalado FastComments a través de la integración del Vercel Marketplace, el ID de su tenant está disponible como la variable de entorno `FASTCOMMENTS_TENANT_ID`. Para leerlo en el cliente, expóngalo a través de `next.config.js` o añádale el prefijo `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```
---