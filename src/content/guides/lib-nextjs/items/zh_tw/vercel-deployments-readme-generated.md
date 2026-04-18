如果您已透過 Vercel Marketplace 的整合安裝 FastComments，您的租戶 ID 可作為 `FASTCOMMENTS_TENANT_ID` 環境變數取得。要在客戶端讀取它，請透過 `next.config.js` 將它暴露出來，或在變數名前加上 `NEXT_PUBLIC_`：

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```