---
## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Non |  |
| userId | string | Non |  |
| startDate | string | Non |  |
| questionId | string | Non |  |
| questionIds | string | Non |  |
| skip | number | Non |  |

## Réponse

Renvoie: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const urlId: string = 'news/2025/fastcomments-release';
const userId: string = 'user_8b7f3c';
const startDate: string = '2025-01-01T00:00:00Z';
const questionIds: string = 'q123,q124';
const skip: number = 20;
const result: GetQuestionResults200Response = await getQuestionResults(tenantId, urlId, userId, startDate, undefined, questionIds, skip);
[inline-code-end]

---