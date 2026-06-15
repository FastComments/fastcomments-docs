## パラメーター

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateTenantBody | UpdateTenantBody | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'updateTenant の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f12';
const id: string = 'flag_192b';
const updateTenantBody: UpdateTenantBody = {
  name: 'Acme Media',
  billingInfo: { plan: 'enterprise', seats: 25, nextBillingDate: '2026-07-01' },
  apiDomainConfiguration: { primaryDomain: 'comments.acme.com', additionalDomains: ['acme.com'] },
  importedSites: [{ siteUrl: 'https://blog.acme.com', archived: false }], // 任意
  commentSettings: { htmlRenderingMode: 'sanitized', deletionMode: 'soft' } // 任意
} as UpdateTenantBody;
const result: FlagCommentPublic200Response = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]