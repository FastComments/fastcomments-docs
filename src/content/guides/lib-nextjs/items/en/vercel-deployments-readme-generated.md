If you've installed FastComments through the Vercel Marketplace integration, your tenant ID is available as the `FASTCOMMENTS_TENANT_ID` environment variable. To read it on the client, expose it through `next.config.js` or prefix it with `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```