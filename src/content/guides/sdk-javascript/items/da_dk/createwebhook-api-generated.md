Abonnerer på en URL til en kommentarhændelse (REST hook-abonnement). At abonnere på den samme URL til den samme hændelse og domæne igen returnerer den eksisterende abonnement. Leveringer er HMAC‑signeret, se webhooks‑guiden; den ældre `token`‑header sendes aldrig til API‑abonnementer.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createWebhookParams | CreateWebhookParams | Ja |  |

## Svar

Returnerer: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'createWebhook Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // valgfri hemmelighed til payload‑verifikation
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]