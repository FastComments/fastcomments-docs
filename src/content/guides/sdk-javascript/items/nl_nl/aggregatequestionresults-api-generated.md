## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| questionId | string | Nee |  |
| questionIds | Array<string> | Nee |  |
| urlId | string | Nee |  |
| timeBucket | AggregateTimeBucket | Nee |  |
| startDate | Date | Nee |  |
| forceRecalculate | boolean | Nee |  |

## Respons

Retourneert: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResultsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'aggregateQuestionResults Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runAggregation() {
  const tenantId: string = '123e4567-e89b-12d3-a456-426614174000';
  const questionId: string = 'question-9f8e7d6c5b4a3';
  const questionIds: string[] = ['question-1a2b3c', 'question-4d5e6f'];
  const urlId: string = 'blog-post-2023-08-15';
  const startDate: Date = new Date('2023-01-01T00:00:00Z');
  const forceRecalculate: boolean = true;

  const result: AggregateQuestionResultsResponse = await aggregateQuestionResults(
    tenantId,
    questionId,
    questionIds,
    urlId,
    undefined,
    startDate,
    forceRecalculate
  );
}
[inline-code-end]