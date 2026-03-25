## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Nein |  |
| userId | string | Nein |  |
| startDate | string | Nein |  |
| questionId | string | Nein |  |
| questionIds | string | Nein |  |
| skip | number | Nein |  |

## Antwort

Gibt zurück: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getQuestionResults Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_001";
const urlId: string = "articles/product-launch-2026";
const userId: string = "user_2048";
const startDate: string = "2026-03-01T00:00:00Z";
const questionId: string | undefined = undefined;
const questionIds: string | undefined = "q_101,q_102";
const skip: number | undefined = 0;

const result: GetQuestionResults200Response = await getQuestionResults(tenantId, urlId, userId, startDate, questionId, questionIds, skip);
[inline-code-end]

---