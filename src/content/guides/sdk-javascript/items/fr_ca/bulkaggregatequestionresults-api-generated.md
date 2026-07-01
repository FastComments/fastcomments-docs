## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Oui |  |
| forceRecalculate | boolean | Non |  |

## Réponse

Renvoie : [`BulkAggregateQuestionResultsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResultsResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple bulkAggregateQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";

const request: BulkAggregateQuestionResultsRequest = {
  questionIds: ["product-satisfaction", "feature-usage"],
  startDate: "2024-01-01T00:00:00Z",
  endDate: "2024-01-31T23:59:59Z",
  timeBucket: "day"
};

const result: BulkAggregateQuestionResultsResponse1 = await bulkAggregateQuestionResults(
  tenantId,
  request,
  true
);
[inline-code-end]