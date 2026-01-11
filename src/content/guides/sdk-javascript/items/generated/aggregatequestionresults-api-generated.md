## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| questionId | string | No |  |
| questionIds | Array<string> | No |  |
| urlId | string | No |  |
| timeBucket | AggregateTimeBucket | No |  |
| startDate | Date | No |  |
| forceRecalculate | boolean | No |  |

## Response

Returns: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResults200Response.ts)

## Example

[inline-code-attrs-start title = 'aggregateQuestionResults Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_8a3f9b2";
  const questionId: string | undefined = undefined;
  const questionIds: Array<string> = ["q-2024-001", "q-2024-002"];
  const urlId: string | undefined = "url_4f9c";
  const timeBucket: AggregateTimeBucket = ("monthly" as unknown) as AggregateTimeBucket;
  const startDate: Date = new Date("2025-01-01T00:00:00Z");
  const forceRecalculate: boolean = true;
  const result: AggregateQuestionResults200Response = await aggregateQuestionResults(
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
