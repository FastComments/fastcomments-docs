---
Listet die für den Mandanten konfigurierten Webhooks auf, sowohl dashboard‑verwaltete Zeilen als auch API‑Abonnements.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| event | WebhookEventName | Nein |  |
| domain | string | Nein |  |
| source | WebhookSource | Nein |  |
| skip | number | Nein |  |

## Antwort

Rückgabe: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhooksResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getWebhooks Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_42';
  const event: WebhookEventName = 'comment.created';
  const domain: string = 'forum.example.org';
  const source: WebhookSource = 'admin';
  const skip: number = 0;

  const fullResponse: GetWebhooksResponse = await getWebhooks(tenantId, event, domain, source, skip);
  const minimalResponse: GetWebhooksResponse = await getWebhooks(tenantId);
})();
[inline-code-end]

---