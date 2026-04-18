Ako ste instalirali FastComments putem integracije Vercel Marketplace, vaš tenant ID dostupan je kao varijabla okruženja `FASTCOMMENTS_TENANT_ID`. Da biste ga pročitali na klijentu, izložite ga putem `next.config.js` ili mu dodajte prefiks `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```