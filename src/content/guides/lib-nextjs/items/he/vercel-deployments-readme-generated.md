אם התקנת את FastComments באמצעות האינטגרציה של Vercel Marketplace, מזהה ה-tenant שלך זמין כמשתנה סביבה `FASTCOMMENTS_TENANT_ID`. כדי לקרוא אותו בצד הלקוח, חשוף אותו באמצעות `next.config.js` או הוסף לו את הקידומת `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```