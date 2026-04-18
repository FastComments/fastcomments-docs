Ako ste instalirali FastComments putem Vercel Marketplace integracije, vaš tenant ID je dostupan kao promenljiva okruženja `FASTCOMMENTS_TENANT_ID`. Da biste je pročitali na klijentu, izložite je putem `next.config.js` ili joj dodajte prefiks `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```