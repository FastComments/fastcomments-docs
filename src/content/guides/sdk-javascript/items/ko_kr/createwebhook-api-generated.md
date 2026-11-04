URL을 댓글 이벤트에 구독시킵니다(REST hook 구독). 동일한 URL을 동일한 이벤트와 도메인에 다시 구독하면 기존 구독이 반환됩니다. 전송은 HMAC으로 서명됩니다. 자세한 내용은 웹훅 가이드를 참조하세요. 레거시 `token` 헤더는 API 구독에 절대 전송되지 않습니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createWebhookParams | CreateWebhookParams | 예 |  |

## 응답

반환값: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## 예제

[inline-code-attrs-start title = 'createWebhook 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // 페이로드 검증을 위한 선택적 시크릿
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]