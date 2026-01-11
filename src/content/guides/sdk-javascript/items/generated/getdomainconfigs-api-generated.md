## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |

## Response

Returns: [`GetDomainConfigs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetDomainConfigs200Response.ts)

## Example

[inline-code-attrs-start title = 'getDomainConfigs Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-marketing-1a2b';
const includeInactive: boolean | undefined = undefined; // optional parameter demonstration
const options: { includeInactive?: boolean } = includeInactive === undefined ? {} : { includeInactive };
const response: GetDomainConfigs200Response = await getDomainConfigs(tenantId);
/* Optional parameters can be supplied as a second argument when available, e.g.:
   await getDomainConfigs(tenantId, { includeInactive: true }) */
[inline-code-end]
