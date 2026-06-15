## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却値: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## 例

[inline-code-attrs-start title = 'pinComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-72";
const commentId: string = "cmt_8f3a2b4c9d";
const broadcastId: string = "live_2026-06-15_21z";
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload.signature";

const responseNoSSO: PinComment200Response = await pinComment(tenantId, commentId, broadcastId);
const responseWithSSO: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, ssoToken);
[inline-code-end]

---