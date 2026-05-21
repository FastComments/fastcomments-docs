## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| domain | string | Yes |  |

## Response

Returns: [`DeleteDomainConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteDomainConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteDomainConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';
const domain: string = 'auth.acmecorp.io';
const options: { notifyAdmin?: boolean } = { notifyAdmin: true };
const result: DeleteDomainConfig200Response = await deleteDomainConfig(tenantId, domain);
[inline-code-end]
