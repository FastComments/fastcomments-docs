Εάν έχετε εγκαταστήσει το FastComments μέσω της ενσωμάτωσης στο Vercel Marketplace, το tenant ID σας είναι διαθέσιμο ως μεταβλητή περιβάλλοντος `FASTCOMMENTS_TENANT_ID`. Για να το διαβάσετε στον client, εκθέστε το μέσω του `next.config.js` ή προσθέστε το πρόθεμα `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```