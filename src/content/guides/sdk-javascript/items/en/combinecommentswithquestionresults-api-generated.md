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
const tenantId: string = 'tenant_abc123';
const questionIds: string[] = ['q_101', 'q_102'];
const startDate: Date = new Date('2025-01-01T00:00:00Z');
const limit: number = 200;
const result: CombineCommentsWithQuestionResults200Response = await combineCommentsWithQuestionResults({
  tenantId,
  questionIds,
  startDate,
  forceRecalculate: true,
  minValue: 0,
  maxValue: 100,
  limit
});
[inline-code-end]
