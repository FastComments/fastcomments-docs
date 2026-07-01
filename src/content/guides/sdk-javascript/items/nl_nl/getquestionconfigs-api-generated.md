## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Respons

Returned: [`GetQuestionConfigsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchQuestionConfigs() {
    const tenantId: string = "tenant-9876";

    // Call without the optional 'skip' parameter
    const configsWithoutSkip: GetQuestionConfigsResponse1 = await getQuestionConfigs(tenantId);

    // Call with the optional 'skip' parameter
    const skip: number = 10;
    const configsWithSkip: GetQuestionConfigsResponse1 = await getQuestionConfigs(tenantId, skip);

    console.log(configsWithoutSkip, configsWithSkip);
}
[inline-code-end]