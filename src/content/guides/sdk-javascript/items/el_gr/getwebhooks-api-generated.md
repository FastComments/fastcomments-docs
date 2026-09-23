Καταγράφει τα webhooks που έχουν ρυθμιστεί για τον ενοικιαστή, τόσο τις γραμμές που διαχειρίζονται μέσω του πίνακα ελέγχου όσο και τις συνδρομές API.

## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| event | WebhookEventName | Όχι |  |
| domain | string | Όχι |  |
| source | WebhookSource | Όχι |  |
| skip | number | Όχι |  |

## Response

Returns: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhooksResponse.ts)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getWebhooks'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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