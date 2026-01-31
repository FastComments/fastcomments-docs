## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| questionId | string | No |  |
| questionIds | Array<string> | No |  |
| urlId | string | No |  |
| startDate | Date | No |  |
| forceRecalculate | boolean | No |  |
| minValue | number | No |  |
| maxValue | number | No |  |
| limit | number | No |  |

## Response

Returns: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineCommentsWithQuestionResults200Response.ts)

## Example

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_12345';
const questionId: string | undefined = undefined; // optional, omitted here
const questionIds: string[] = ['q_abc123', 'q_def456'];
const urlId: string = 'article-2025-01-09';
const startDate: Date = new Date('2025-01-01T00:00:00Z');
const forceRecalculate: boolean = true;
const minValue: number = 1;
const maxValue: number = 5;
const limit: number = 50;

const result: CombineCommentsWithQuestionResults200Response = await combineCommentsWithQuestionResults(
  tenantId,
  questionId,
  questionIds,
  urlId,
  startDate,
  forceRecalculate,
  minValue,
  maxValue,
  limit
);
[inline-code-end]
