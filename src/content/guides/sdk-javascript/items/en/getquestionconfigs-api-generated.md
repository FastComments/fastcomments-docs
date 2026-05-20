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
async function run(): Promise<void> {
  const tenantId: string = "acme-corp-987";
  const skip: number = 20;
  const resultWithSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId, skip);
  const resultWithoutSkip: GetQuestionConfigs200Response = await getQuestionConfigs(tenantId);
}
run();
[inline-code-end]
