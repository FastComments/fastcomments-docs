Če ste namestili FastComments preko integracije Vercel Marketplace, je vaš ID najemnika na voljo kot okoljska spremenljivka `FASTCOMMENTS_TENANT_ID`. Če ga želite prebrati na odjemalcu, ga izpostavite v `next.config.js` ali mu dodajte predpono `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```