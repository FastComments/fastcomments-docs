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
const tenantId: string = 'acme-corp-tenant-9a8b';
const userId: string = 'user_73f2b6';
const sessionSegment: string | undefined = undefined; // optional segment appended when present
const effectiveId: string = sessionSegment ? `${userId}:${sessionSegment}` : userId;
const result: GetSSOUserByIdAPIResponse = await getSSOUserById(tenantId, effectiveId);
[inline-code-end]
