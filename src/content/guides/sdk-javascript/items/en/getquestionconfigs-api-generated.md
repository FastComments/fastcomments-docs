## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigs200Response.ts)

## Example

[inline-code-attrs-start title = 'getQuestionConfigs Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant-8f4b2c';
  const skip: number = 40;
  const responseWithoutSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
  const responseWithSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, skip);
  console.log(responseWithoutSkip, responseWithSkip);
})();
[inline-code-end]
