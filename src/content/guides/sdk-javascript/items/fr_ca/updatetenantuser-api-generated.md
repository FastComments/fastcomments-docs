## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateTenantUserBody | UpdateTenantUserBody | Oui |  |
| updateComments | string | Non |  |

## Réponse

Renvoie: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3b2a9d";
const id: string = "user_52c9f1ab";
const updateTenantUserBody: UpdateTenantUserBody = {
  email: "jane.doe@example.com",
  displayName: "Jane Doe",
  roles: ["moderator"],
  isActive: true,
  metadata: { signupSource: "sso", locale: "en-US" }
};
const updateComments: string = "Promoted to moderator and updated display name";
const result: FlagCommentPublic200Response = await updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
[inline-code-end]

---