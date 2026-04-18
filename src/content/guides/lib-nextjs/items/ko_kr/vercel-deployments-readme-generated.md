Vercel Marketplace 통합을 통해 FastComments를 설치한 경우, 테넌트 ID는 `FASTCOMMENTS_TENANT_ID` 환경 변수로 제공됩니다. 클라이언트에서 읽으려면 `next.config.js`를 통해 노출하거나 `NEXT_PUBLIC_` 접두사를 붙이세요:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```