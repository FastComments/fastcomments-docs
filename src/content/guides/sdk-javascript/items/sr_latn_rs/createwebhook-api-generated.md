Pretplaćuje URL na događaj komentara (REST hook subscribe). Pretplaćivanje istog URL-a na isti  
događaj i domen ponovo vraća postojeću pretplatu. Dostave su HMAC potpisane, pogledajte  
vodič za webhooks; legacy `token` zaglavlje se nikada ne šalje API pretplatama.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createWebhookParams | CreateWebhookParams | Yes |  |

## Response

Returns: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Example

[inline-code-attrs-start title = 'createWebhook Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // opcioni tajni ključ za verifikaciju payload-a
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]