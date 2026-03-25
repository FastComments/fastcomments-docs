## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| postIds | Array<string> | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却値: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsStats200Response.ts)

## 例

[inline-code-attrs-start title = 'getFeedPostsStats の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9b2f1c4a';
const postIds: Array<string> = [
  '8f14e45f-ea82-4c7a-b6b2-1a2b3c4d5e6f',
  'd0e1f2a3-b4c5-6d7e-8f90-1234567890ab'
];
const sso: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signature';
const statsWithoutSSO: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds);
const statsWithSSO: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds, sso);
[inline-code-end]

---