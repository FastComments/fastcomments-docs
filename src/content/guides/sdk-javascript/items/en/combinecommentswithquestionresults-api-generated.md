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
(async () => {
  const tenantId: string = 'tenant_3f9b2a';
  const questionId: string = 'q_1024';
  const questionIds: string[] = ['q_1024', 'q_2048'];
  const urlId: string = 'url_77';
  const startDate: Date = new Date('2026-01-01T00:00:00Z');
  const forceRecalculate: boolean = true;
  const minValue: number = 0;
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
  console.log(result);
})();
[inline-code-end]
