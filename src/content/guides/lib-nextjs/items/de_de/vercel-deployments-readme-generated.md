Wenn Sie FastComments über die Vercel Marketplace-Integration installiert haben, ist Ihre Tenant-ID als Umgebungsvariable `FASTCOMMENTS_TENANT_ID` verfügbar. Um sie im Client auszulesen, machen Sie sie über `next.config.js` verfügbar oder versehen Sie sie mit dem Präfix `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```