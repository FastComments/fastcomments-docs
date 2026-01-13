## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| updateDomainConfigParams | UpdateDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'putDomainConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_92b4c1a7";
const domainToUpdate: string = "blog.company-example.com";
const updateDomainConfigParams: UpdateDomainConfigParams = {
  enableComments: true,
  moderationMode: "pre-moderation",
  allowedOrigins: ["https://company-example.com", "https://studio.company-example.com"],
  // optional parameter demonstrated by including redirectToCanonical (may be omitted)
  redirectToCanonical: true
};
const response: GetDomainConfig200Response = await putDomainConfig(tenantId, domainToUpdate, updateDomainConfigParams);
[inline-code-end]
