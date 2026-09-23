Abonniert eine URL für ein Kommentarereignis (REST‑Hook‑Abonnement). Das erneute Abonnieren derselben URL für dasselbe Ereignis und dieselbe Domain gibt das bestehende Abonnement zurück. Lieferungen sind HMAC‑signiert, siehe die Webhooks‑Anleitung; der veraltete `token`‑Header wird bei API‑Abonnements niemals gesendet.

## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| createWebhookParams | CreateWebhookParams | Ja |  |

## Antwort

Rückgabe: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'createWebhook Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // optionaler Geheimschlüssel zur Payload‑Überprüfung
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]

---