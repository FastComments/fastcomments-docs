## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPISSOUserData | CreateAPISSOUserData | Yes |  |

## Response

Returns: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddSSOUserAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'addSSOUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const newUser: CreateAPISSOUserData = {
  userId: "sso_user_987",
  name: "Jane Doe",
  email: "jane.doe@example.com",
  // optional field
  avatarUrl: "https://example.com/avatars/jane.jpg",
};

const result: AddSSOUserAPIResponse = await addSSOUser(tenantId, newUser);
[inline-code-end]
