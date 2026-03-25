## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`PinComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PinComment200Response.ts)

## 示例

[inline-code-attrs-start title = 'pinComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4f2b9a";
const commentId: string = "cmt_9f8e7d6c";
const broadcastId: string = "brd_live_concert_2026-03-25";
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.sso_payload_signature";

const result: PinComment200Response = await pinComment(tenantId, commentId, broadcastId, sso);
[inline-code-end]

---