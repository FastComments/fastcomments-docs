## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| questionId | string | Nein |  |
| questionIds | Array<string> | Nein |  |
| urlId | string | Nein |  |
| timeBucket | AggregateTimeBucket | Nein |  |
| startDate | Date | Nein |  |
| forceRecalculate | boolean | Nein |  |

## Antwort

Rückgabe: [`AggregateQuestionResultsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResultsResponse1.ts)

## Beispiel

[inline-code-attrs-start title = 'aggregateQuestionResults Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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