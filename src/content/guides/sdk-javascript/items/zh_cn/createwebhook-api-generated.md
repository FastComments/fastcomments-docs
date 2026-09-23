订阅一个 URL 到评论事件（REST hook 订阅）。再次将相同的 URL 订阅到相同的事件和域时，将返回已有的订阅。交付内容使用 HMAC 签名，参见 webhooks 指南；旧版 `token` 头部永不发送到 API 订阅。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| createWebhookParams | CreateWebhookParams | 是 |  |

## 响应

返回：[`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## 示例

[inline-code-attrs-start title = 'createWebhook 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // optional secret for payload verification
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]