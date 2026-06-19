## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| questionId | string | Όχι |  |
| questionIds | Array<string> | Όχι |  |
| urlId | string | Όχι |  |
| startDate | Date | Όχι |  |
| forceRecalculate | boolean | Όχι |  |
| minValue | number | Όχι |  |
| maxValue | number | Όχι |  |
| limit | number | Όχι |  |

## Απόκριση

Επιστρέφει: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineQuestionResultsWithCommentsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα combineCommentsWithQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7c9f4b3a';
const questionIds: string[] = ['q-4f8b2a1c', 'q-9d3e7b0f'];
const urlId: string = 'url_93b2c1a7';
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;
const minValue: number = 0.2;
const maxValue: number = 0.95;
const limit: number = 100;

const combinedResult: CombineQuestionResultsWithCommentsResponse = await combineCommentsWithQuestionResults({
  tenantId,
  questionIds,
  urlId,
  startDate,
  forceRecalculate,
  minValue,
  maxValue,
  limit
});
[inline-code-end]