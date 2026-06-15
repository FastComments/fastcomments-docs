## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Nee |  |
| userId | string | Nee |  |
| startDate | string | Nee |  |
| questionId | string | Nee |  |
| questionIds | string | Nee |  |
| skip | number | Nee |  |

## Antwoord

Retourneert: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getQuestionResults Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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