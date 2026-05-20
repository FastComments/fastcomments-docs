## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | No |  |
| userId | string | No |  |
| startDate | string | No |  |
| questionId | string | No |  |
| questionIds | string | No |  |
| skip | number | No |  |

## Response

Returns: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## Example

[inline-code-attrs-start title = 'getQuestionResults Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-72';
const urlId: string = 'survey-2025-03';
const userId: string | undefined = undefined;
const startDate: string = '2024-01-01T00:00:00Z';
const questionId: string | undefined = undefined;
const questionIds: string = 'Q-718,Q-719';
const skip: number = 20;

const result: GetQuestionResults200Response = await getQuestionResults(
  tenantId,
  urlId,
  userId,
  startDate,
  questionId,
  questionIds,
  skip
);
[inline-code-end]
