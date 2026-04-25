---
## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| createQuestionResultBody | CreateQuestionResultBody | Tak |  |

## Odpowiedź

Zwraca: [`CreateQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResult200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład createQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-01';
const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: 'q-34567',
  respondentId: 'user-8923',
  answers: [{ optionId: 'opt_A', text: 'Agree', count: 1 }],
  score: 5,
  meta: [{ key: 'platform', value: 'web' }],
  notifyModerators: false // opcjonalny parametr
} as CreateQuestionResultBody;
const result: CreateQuestionResult200Response = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]

---