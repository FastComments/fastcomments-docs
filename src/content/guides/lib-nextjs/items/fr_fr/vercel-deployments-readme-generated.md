Si vous avez installé FastComments via l'intégration du Vercel Marketplace, votre ID de locataire est disponible en tant que variable d'environnement `FASTCOMMENTS_TENANT_ID`. Pour la lire côté client, exposez-la via `next.config.js` ou préfixez-la par `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```