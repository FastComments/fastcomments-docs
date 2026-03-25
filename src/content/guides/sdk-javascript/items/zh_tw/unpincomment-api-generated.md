## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## 範例

[inline-code-attrs-start title = 'unPinComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9d2a3b';
const commentId: string = 'comment_842b9c1f';
const broadcastId: string = 'bcast_frontpage_202603';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature';

const result: PinComment200Response = await unPinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---