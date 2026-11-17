Πρόσφατα σχόλια ακριβώς στη μορφή που χρησιμοποιούν οι παραδόσεις webhook, για την κατασκευή ενσωματώσεων (για παράδειγμα δοκιμαστικά δεδομένα Zapier). Κάθε event παραδίδει το ίδιο αντικείμενο σχολίου, οπότε το `event` χρειάζεται μόνο να είναι έγκυρο.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| event | WebhookEventName | Όχι |  |
| limit | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getWebhookSamplePayloads(
  tenantId: string,
  event?: WebhookEventName,
  limit?: number
): Promise<GetWebhookSamplePayloadsResponse>;

const tenantId: string = '5f8d9b3e2a1b4c6d7e8f9a0b';

// Λήψη δοκιμαστικών payloads για συγκεκριμένο τύπο event
const commentPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  WebhookEventName.COMMENT_SUCCESS_PAGE,
  10
);

// Λήψη όλων των δοκιμαστικών payloads (χωρίς προαιρετικές παραμέτρους)
const allPayloads: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(tenantId);

console.log(`Comment payload status: ${commentPayloads.status}`);
console.log(`Fetched ${allPayloads.payloads.length} sample payloads`);
[inline-code-end]