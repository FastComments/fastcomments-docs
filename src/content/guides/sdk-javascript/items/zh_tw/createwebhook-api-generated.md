訂閱一個 URL 到評論事件（REST hook 訂閱）。再次將相同的 URL 訂閱到相同的事件和網域時，會返回現有的訂閱。傳遞內容使用 HMAC 簽名，請參閱 webhooks 指南；舊版的 `token` 標頭永遠不會發送給 API 訂閱。

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createWebhookParams | CreateWebhookParams | Yes |  |

## Response

Returns: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Example

[inline-code-attrs-start title = 'createWebhook 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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