## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUserByIdAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'getSSOUserById Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'd3f9a8b2-6e11-4c2b-9a7f-5b2e1c3d4f6a';
const id: string = '55a3c9e2-1b44-4f3a-9b8c-2d7e6f1a9b0c';
const ssoResponse: GetSSOUserByIdAPIResponse = await getSSOUserById(tenantId, id);
let apiUser: APISSOUser | undefined = undefined;
async function fetchSSOOptional(tenantId: string, id?: string): Promise<GetSSOUserByIdAPIResponse | undefined> {
  if (!id) return undefined;
  const res: GetSSOUserByIdAPIResponse = await getSSOUserById(tenantId, id);
  return res;
}
const maybeResponse: GetSSOUserByIdAPIResponse | undefined = await fetchSSOOptional(tenantId, undefined);
[inline-code-end]
