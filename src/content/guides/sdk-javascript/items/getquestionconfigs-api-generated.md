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
const tenantId: string = "tenant-4f1a9b2c";
const responseDefault: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
const responsePaged: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, 50);
[inline-code-end]
