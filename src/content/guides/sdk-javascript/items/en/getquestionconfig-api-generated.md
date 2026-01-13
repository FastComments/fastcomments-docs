## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'getQuestionConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-47';
const questionId: string = 'q-2026-01-12-01';
const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, questionId);
function summarizeConfig(cfg: GetQuestionConfig200Response, includeMetadata?: boolean): QuestionConfig | undefined {
  // includeMetadata is optional; implementation omitted for brevity
  return undefined;
}
const summarizedWithMeta: QuestionConfig | undefined = summarizeConfig(result, true);
const summarizedDefault: QuestionConfig | undefined = summarizeConfig(result);
[inline-code-end]
