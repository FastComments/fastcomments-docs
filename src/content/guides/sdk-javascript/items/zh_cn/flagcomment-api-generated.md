---
## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |

## 响应

返回: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## 示例

[inline-code-attrs-start title = 'flagComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b21';
const commentId: string = 'cmt_9a2b4';
const userId: string = 'user_1024';
const result: FlagComment200Response = await flagComment(tenantId, commentId, userId);
[inline-code-end]

---