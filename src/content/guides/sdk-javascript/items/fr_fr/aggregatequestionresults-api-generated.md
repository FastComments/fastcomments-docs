## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| questionId | string | Non |  |
| questionIds | Array<string> | Non |  |
| urlId | string | Non |  |
| timeBucket | AggregateTimeBucket | Non |  |
| startDate | Date | Non |  |
| forceRecalculate | boolean | Non |  |

## Réponse

Retourne : [`AggregateQuestionResultsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResultsResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'aggregateQuestionResults Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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