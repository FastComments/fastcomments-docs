如果你通过 Vercel Marketplace 集成安装了 FastComments，你的租户 ID 可作为环境变量 `FASTCOMMENTS_TENANT_ID` 使用。要在客户端读取它，请通过 `next.config.js` 将其公开，或在其前添加 `NEXT_PUBLIC_` 前缀：

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```