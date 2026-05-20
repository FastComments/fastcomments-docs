## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| sure | string | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteTenant Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'acme-tenant-48';
  const id: string = 'flag-9001';
  const sure: string = 'confirm-delete';
  const resultWithSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id, sure);
  const resultWithoutSure: FlagCommentPublic200Response = await deleteTenant(tenantId, id);
  console.log(resultWithSure, resultWithoutSure);
})();
[inline-code-end]
