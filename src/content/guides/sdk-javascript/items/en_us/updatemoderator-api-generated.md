## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateModeratorBody | UpdateModeratorBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateModerator Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises-42";
const id: string = "moderator_517";
const updateModeratorBody: UpdateModeratorBody = {
  displayName: "Sofia Martinez",
  email: "sofia.martinez@acme.com",
  permissions: ["approve_comments", "flag_spam", "suspend_users"],
  active: true,
  avatarUrl: "https://cdn.acme.com/avatars/sofia.jpg" // optional field demonstrated
};
const result: FlagCommentPublic200Response = await updateModerator(tenantId, id, updateModeratorBody);
[inline-code-end]
