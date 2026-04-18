Se hai installato FastComments tramite l'integrazione del Marketplace di Vercel, il tuo ID del tenant è disponibile come variabile d'ambiente `FASTCOMMENTS_TENANT_ID`. Per leggerlo nel client, esponilo tramite `next.config.js` o anteponi `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```