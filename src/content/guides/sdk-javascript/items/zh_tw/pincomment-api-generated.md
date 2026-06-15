## 參數

| 名稱 | 類型 | 必需 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## 範例

[inline-code-attrs-start title = 'pinComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-72";
const commentId: string = "cmt_8f3a2b4c9d";
const broadcastId: string = "live_2026-06-15_21z";
const ssoToken: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload.signature";

const responseNoSSO: PinComment200Response = await pinComment(tenantId, commentId, broadcastId);
const responseWithSSO: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, ssoToken);
[inline-code-end]

---