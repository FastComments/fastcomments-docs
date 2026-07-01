## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createAPISSOUserData | CreateAPISSOUserData | Da |  |

## Odgovor

Returns: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddSSOUserAPIResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer addSSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const newUser: CreateAPISSOUserData = {
  userId: "sso_user_987",
  name: "Jane Doe",
  email: "jane.doe@example.com",
  // opcijsko polje
  avatarUrl: "https://example.com/avatars/jane.jpg",
};

const result: AddSSOUserAPIResponse = await addSSOUser(tenantId, newUser);
[inline-code-end]

---