Subscribes a URL to a comment event (REST hook subscribe). Subscribing the same URL to the same
event and domain again returns the existing subscription. Deliveries are HMAC signed, see the
webhooks guide; the legacy `token` header is never sent to API subscriptions.

## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createWebhookParams | CreateWebhookParams | Yes |  |

## Response

반환: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## 예시

[inline-code-attrs-start title = 'createWebhook 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // 페이로드 검증을 위한 선택적 비밀키
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]