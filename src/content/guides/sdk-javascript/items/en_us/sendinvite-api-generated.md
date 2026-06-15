## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| fromName | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'sendInvite Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_acme_42";
  const id: string = "cmt_8f3b21";
  const fromName: string = "Ava Thompson";
  const inviteResult: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName);
  console.log(inviteResult);
})();
[inline-code-end]
