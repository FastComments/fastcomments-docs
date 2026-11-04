Bir URL'yi bir yorum olayına abone eder (REST hook subscribe). Aynı URL'yi aynı olay ve alan adına (domain) tekrar abone etmek, mevcut aboneliği döndürür. Teslimatlar HMAC ile imzalanır, webhooks rehberine bakın; eski `token` başlığı API aboneliklerine asla gönderilmez.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createWebhookParams | CreateWebhookParams | Evet |  |

## Yanıt

Döndürür: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Örnek

[inline-code-attrs-start title = 'createWebhook Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // payload doğrulaması için isteğe bağlı gizli anahtar
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]