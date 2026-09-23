Πρόσφατα σχόλια ακριβώς με τη μορφή που χρησιμοποιούν οι παραδόσεις webhook, για τη δημιουργία ενσωματώσεων (για παράδειγμα δείγματα δεδομένων Zapier). Κάθε γεγονός παραδίδει το ίδιο αντικείμενο σχολίου, έτσι το `event` πρέπει μόνο να είναι έγκυρο.

## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| event | WebhookEventName | Όχι |  |
| limit | number | Όχι |  |

## Response

Επιστρέφει: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhookSamplePayloadsResponse.ts)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getWebhookSamplePayloads'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c1a2b3d4-5678-90ab-cdef-1234567890ab";

const responseAll: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created",
  10
);

const responseWithEvent: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId,
  "comment.created"
);

const responseBasic: GetWebhookSamplePayloadsResponse = await getWebhookSamplePayloads(
  tenantId
);
[inline-code-end]