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
const tenantId: string = 'acme-enterprises-us';
const id: string = 'qc-7b3a9f2';
const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, id);
const customOptions: QuestionConfigCustomOptionsInner[] | undefined = result.questionConfig?.customOptions;
console.log(`Tenant=${tenantId} id=${id} customOptionsCount=${customOptions?.length ?? 0}`);
[inline-code-end]
