Subskrybuje URL do zdarzenia komentarza (subskrypcja REST hook). Ponowne subskrybowanie tego samego URL do tego samego zdarzenia i domeny zwraca istniejącą subskrypcję. Dostawy są podpisane HMAC, zobacz przewodnik po webhookach; starszy nagłówek `token` nigdy nie jest wysyłany do subskrypcji API.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createWebhookParams | CreateWebhookParams | Yes |  |

## Response

Returns: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Example

[inline-code-attrs-start title = 'Przykład createWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // opcjonalny secret do weryfikacji ładunku
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]

---