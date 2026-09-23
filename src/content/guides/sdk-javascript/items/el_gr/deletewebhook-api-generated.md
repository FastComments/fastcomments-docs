---
Αποεγγραφές (REST hook unsubscribe). Μόνο οι συνδρομές που δημιουργήθηκαν μέσω αυτού του API μπορούν να
διαγραφούν εδώ· οι webhook που διαχειρίζονται από τον πίνακα ελέγχου επεξεργάζονται στον πίνακα ελέγχου.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Response

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'Παράδειγμα deleteWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const webhookId: string = "wh_98765";

const result: APIEmptyResponse = await deleteWebhook(tenantId, webhookId);
[inline-code-end]

---