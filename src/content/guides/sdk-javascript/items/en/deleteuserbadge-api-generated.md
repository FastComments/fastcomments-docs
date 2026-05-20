## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteUserBadge Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'olympus_inc';
  const id: string = 'badge-73a9b2';
  const includeDetails: boolean | undefined = true;
  const response: UpdateUserBadge200Response = await deleteUserBadge(tenantId, id);
  console.log(response);
})();
[inline-code-end]
