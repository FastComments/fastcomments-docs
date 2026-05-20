## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenantUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const regionSuffix: string | undefined = '-west';
const tenantId: string = `acme-corp${regionSuffix ?? ''}`;
const userId: string = 'u-82b7f3c9';
const result: GetTenantUser200Response = await getTenantUser(tenantId, userId);
[inline-code-end]
