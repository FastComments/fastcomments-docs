## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## Example

[inline-code-attrs-start title = 'getUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantOverride: string | undefined = undefined;
const tenantId: string = tenantOverride ?? 'tenant_9a4f';
const userId: string = 'u_7b3c';
const result: GetUser200Response = await getUser(tenantId, userId);
[inline-code-end]
