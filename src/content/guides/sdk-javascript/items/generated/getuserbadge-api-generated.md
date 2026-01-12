## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserBadge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoFetch(tenantId: string, id: string, includeRelated?: boolean): Promise<GetUserBadge200Response> {
  const response: GetUserBadge200Response = await getUserBadge(tenantId, id);
  return response;
}

const tenantId: string = 'acme-enterprises-78';
const badgeId: string = '9f8b7c6d-5e4f-4012-8a3b-0c1d2e3f4a5b';
const badgeResponse: GetUserBadge200Response = await getUserBadge(tenantId, badgeId);
[inline-code-end]
