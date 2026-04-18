Hvis du har installeret FastComments gennem Vercel Marketplace-integrationen, er dit tenant-id tilgængeligt som miljøvariablen `FASTCOMMENTS_TENANT_ID`. For at læse det på klienten, eksponer det gennem `next.config.js` eller giv det præfikset `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```