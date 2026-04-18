Si vous avez installé FastComments via l'intégration du Marketplace Vercel, l'identifiant de votre locataire est disponible dans la variable d'environnement `FASTCOMMENTS_TENANT_ID`. Pour le lire côté client, exposez-le via `next.config.js` ou préfixez-le par `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```