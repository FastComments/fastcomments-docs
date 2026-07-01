## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetQuestionResultResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultResponse1.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchQuestionResult(): Promise<void> {
    const tenantId: string = "acme-corp-001";
    const questionId: string = "question-7a9b8c";
    const result: GetQuestionResultResponse1 = await getQuestionResult(tenantId, questionId);

    const question: QuestionResult | undefined = result.questionResult;
    const firstMeta: MetaItem | undefined = result.meta?.[0];

    console.log(question?.id, firstMeta?.key);
}
[inline-code-end]