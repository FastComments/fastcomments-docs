## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| questionId | string | Ne |  |
| questionIds | Array<string> | Ne |  |
| urlId | string | Ne |  |
| timeBucket | AggregateTimeBucket | Ne |  |
| startDate | Date | Ne |  |
| forceRecalculate | boolean | Ne |  |

## Odgovor

Vraća: [`AggregateQuestionResultsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResultsResponse1.ts)

## Primjer

[inline-code-attrs-start title = 'aggregateQuestionResults Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-123";
  const questionId: string = "feedback-rating";
  const questionIds: string[] = ["feedback-rating", "recommendation"];
  const urlId: string = "https://myblog.com/articles/42";
  const timeBucket: AggregateTimeBucket = "month";
  const startDate: Date = new Date("2023-01-01T00:00:00Z");
  const forceRecalculate: boolean = true;

  const result: AggregateQuestionResultsResponse1 = await aggregateQuestionResults(
    tenantId,
    questionId,
    questionIds,
    urlId,
    timeBucket,
    startDate,
    forceRecalculate
  );

  console.log(result);
})();
[inline-code-end]

---