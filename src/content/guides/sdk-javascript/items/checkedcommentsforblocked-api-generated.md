## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentIds | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CheckedCommentsForBlocked200Response.ts)

## Example

[inline-code-attrs-start title = 'checkedCommentsForBlocked Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-4f7b2a';
const commentIds: string = 'cmt-10293,cmt-10458';
const resultWithoutSso: CheckedCommentsForBlocked200Response = await checkedCommentsForBlocked(tenantId, commentIds);
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.dXNlcklkOjEyMzQ.signature';
const resultWithSso: CheckedCommentsForBlocked200Response = await checkedCommentsForBlocked(tenantId, commentIds, ssoToken);
[inline-code-end]
