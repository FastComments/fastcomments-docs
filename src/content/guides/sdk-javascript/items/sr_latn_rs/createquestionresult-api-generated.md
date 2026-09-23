## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createQuestionResultBody | CreateQuestionResultBody | Da |  |

## Odgovor

Vraća: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionResultResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer createQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const createQuestionResultBody: CreateQuestionResultBody = {
  questionId: "q_9876",
  answer: "Yes",
  score: 5,
  meta: [
    { key: "source", value: "survey" }
  ]
};

const result: CreateQuestionResultResponse = await createQuestionResult(tenantId, createQuestionResultBody);
[inline-code-end]