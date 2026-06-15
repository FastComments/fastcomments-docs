## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Non |  |
| userId | string | Non |  |
| startDate | string | Non |  |
| questionId | string | Non |  |
| questionIds | string | Non |  |
| skip | number | Non |  |

## Réponse

Renvoie : [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fc-tenant-7a9c';
const urlId: string = 'news/article-2026-06-15';
const userId: string = 'user-8421';
const startDate: string = '2026-05-01T00:00:00Z';
const questionId: string = 'q-13';
const questionIds: string = 'q-13,q-14';
const skip: number = 20;
const result: GetQuestionResults200Response = await getQuestionResults(tenantId, urlId, userId, startDate, questionId, questionIds, skip);
[inline-code-end]

---