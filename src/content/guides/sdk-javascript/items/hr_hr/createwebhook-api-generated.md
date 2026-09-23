Pretplaćuje URL na događaj komentara (REST hook pretplata). Ponovno pretplaćivanje istog URL-a na isti događaj i domenu vraća postojeću pretplatu. Dostave su HMAC potpisane, pogledajte vodič za webhooks; naslijeđeno `token` zaglavlje se nikada ne šalje API pretplatama.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| createWebhookParams | CreateWebhookParams | Da |  |

## Odgovor

Vraća: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Primjer

[inline-code-attrs-start title = 'createWebhook Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // opcionalni tajni za provjeru payloada
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]