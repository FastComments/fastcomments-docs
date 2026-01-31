## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domainToUpdate | string | Yes |  |
| patchDomainConfigParams | PatchDomainConfigParams | Yes |  |

## Response

Returns: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'patchDomainConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f1c3bcd';
const domainToUpdate: string = 'comments.mycompany.com';
const patchDomainConfigParams: PatchDomainConfigParams = {
  allowedOrigins: ['https://www.mycompany.com', 'https://admin.mycompany.com'],
  enableCommenting: true,
  moderation: { enabled: true, autoApproveTrustedUsers: false } // optional nested settings demonstrated
};
const result: GetDomainConfig200Response = await patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
[inline-code-end]
