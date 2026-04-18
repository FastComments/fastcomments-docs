Als je FastComments via de Vercel Marketplace-integratie hebt geïnstalleerd, is je tenant-ID beschikbaar als de `FASTCOMMENTS_TENANT_ID` omgevingsvariabele. Om deze aan de clientzijde te lezen, maak je deze beschikbaar via `next.config.js` of voeg je het voorvoegsel `NEXT_PUBLIC_` toe:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```