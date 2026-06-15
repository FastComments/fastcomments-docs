## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 示例

[inline-code-attrs-start title = 'deletePendingWebhookEvent 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_5f8d7a34";
const id: string = "webhook_evt_987654321";
const requestNote: string | undefined = undefined;
const response: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, id);
[inline-code-end]

---