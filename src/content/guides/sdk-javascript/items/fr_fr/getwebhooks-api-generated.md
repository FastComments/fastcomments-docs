Répertorie les webhooks configurés pour le locataire, à la fois les lignes gérées via le tableau de bord et les abonnements API.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| event | WebhookEventName | Non |  |
| domain | string | Non |  |
| source | WebhookSource | Non |  |
| skip | number | Non |  |

## Réponse

Renvoie : [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhooksResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getWebhooks'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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