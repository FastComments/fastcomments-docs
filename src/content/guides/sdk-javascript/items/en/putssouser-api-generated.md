## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateAPISSOUserData | UpdateAPISSOUserData | Yes |  |
| updateComments | boolean | No |  |

## Response

Returns: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PutSSOUserAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'putSSOUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fastcomments_tenant_7f3a2b";
const id: string = "sso_user_4a9c21";
const updateData: UpdateAPISSOUserData = {
  email: "marija.kovacevic@example.com",
  displayName: "Marija K.",
  roles: ["moderator"],
  avatarUrl: "https://cdn.example.com/avatars/marija.jpg",
  externalId: "marija-kovacevic-01"
};
const updateComments: boolean = true;
const result: PutSSOUserAPIResponse = await putSSOUser(tenantId, id, updateData, updateComments);
[inline-code-end]
