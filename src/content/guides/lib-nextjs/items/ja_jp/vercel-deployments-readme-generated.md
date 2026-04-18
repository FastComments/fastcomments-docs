Vercel Marketplace 統合を通じて FastComments をインストールした場合、テナント ID は `FASTCOMMENTS_TENANT_ID` 環境変数として利用可能です。クライアント側でそれを読み取るには、`next.config.js` を通じて公開するか、`NEXT_PUBLIC_` をプレフィックスとして付けてください：

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```