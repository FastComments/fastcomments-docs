Αν έχετε εγκαταστήσει το FastComments μέσω της ενσωμάτωσης του Vercel Marketplace, το tenant ID σας είναι διαθέσιμο ως η μεταβλητή περιβάλλοντος `FASTCOMMENTS_TENANT_ID`. Για να το διαβάσετε στην πλευρά του πελάτη, εκθέστε το μέσω του `next.config.js` ή προθέστε το με `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```