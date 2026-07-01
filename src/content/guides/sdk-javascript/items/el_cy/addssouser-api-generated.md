## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPISSOUserData | CreateAPISSOUserData | Yes |  |

## Απάντηση

Επιστρέφει: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AddSSOUserAPIResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα addSSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const newUser: CreateAPISSOUserData = {
  userId: "sso_user_987",
  name: "Jane Doe",
  email: "jane.doe@example.com",
  // προαιρετικό πεδίο
  avatarUrl: "https://example.com/avatars/jane.jpg",
};

const result: AddSSOUserAPIResponse = await addSSOUser(tenantId, newUser);
[inline-code-end]