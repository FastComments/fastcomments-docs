## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createAPISSOUserData | CreateAPISSOUserData | Ja |  |

## Antwort

Returns: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddSSOUserAPIResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'addSSOUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const newUser: CreateAPISSOUserData = {
  userId: "sso_user_987",
  name: "Jane Doe",
  email: "jane.doe@example.com",
  // optionales Feld
  avatarUrl: "https://example.com/avatars/jane.jpg",
};

const result: AddSSOUserAPIResponse = await addSSOUser(tenantId, newUser);
[inline-code-end]

---