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
const tenantId: string = "tenant_6b2f1a2c";
const id: string = "user_73d9e5d4";
const response: GetUser200Response = await getUser(tenantId, id);
async function fetchUserIfPresent(tenantId: string, id?: string): Promise<GetUser200Response | undefined> {
  if (!id) return undefined;
  return await getUser(tenantId, id);
}
const maybeResponse: GetUser200Response | undefined = await fetchUserIfPresent(tenantId, id);
[inline-code-end]
