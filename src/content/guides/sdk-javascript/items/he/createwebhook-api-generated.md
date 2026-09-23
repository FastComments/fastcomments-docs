מנוי כתובת URL לאירוע תגובה (REST hook subscribe). מנוי של אותה כתובת URL לאותו אירוע ולדומיין שוב מחזיר את המנוי הקיים. המשלוחים חתומים ב‑HMAC, ראו את מדריך ה‑webhooks; כותרת `token` הישנה לעולם לא נשלחת למנויים ב‑API.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createWebhookParams | CreateWebhookParams | כן |  |

## תגובה

מחזיר: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת createWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // סוד אופציונלי לאימות המטען
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]