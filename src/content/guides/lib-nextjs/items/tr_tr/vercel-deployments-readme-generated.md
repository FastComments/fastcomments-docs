Eğer FastComments'i Vercel Marketplace entegrasyonu aracılığıyla yüklediyseniz, kiracı kimliğiniz `FASTCOMMENTS_TENANT_ID` ortam değişkeni olarak mevcuttur. İstemci tarafında okumak için, onu `next.config.js` aracılığıyla açığa çıkarın veya `NEXT_PUBLIC_` ile önekleyin:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```