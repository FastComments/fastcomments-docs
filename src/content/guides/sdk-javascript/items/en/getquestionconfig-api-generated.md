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
(async () => {
  const tenantId: string = 'tenant-8a3f';
  const id: string = 'question-5b7c9';
  const region: string | undefined = 'eu-west-1'; // optional parameter example
  const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, id);
  console.log(result);
})();
[inline-code-end]
