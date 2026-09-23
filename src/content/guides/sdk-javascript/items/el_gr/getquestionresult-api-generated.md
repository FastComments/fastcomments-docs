## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απάντηση

Επιστρέφει: [`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const questionId: string = "question_98765";

const response: GetQuestionResultResponse = await getQuestionResult(tenantId, questionId);

const status: APIStatus | undefined = response.status;
const firstResult: QuestionResult | undefined = response.results?.[0];
const firstMeta: MetaItem | undefined = response.meta?.[0];

console.log(response);
[inline-code-end]