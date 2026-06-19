---
## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f2b3c';
const id: string = 'qcfg_9a8b7c';
const metadataNote: string | undefined = undefined; // προαιρετικά μεταδεδομένα (δεν απαιτούνται από τη συνάρτηση)
const result: APIEmptyResponse = await deleteQuestionConfig(tenantId, id);
metadataNote;
[inline-code-end]

---