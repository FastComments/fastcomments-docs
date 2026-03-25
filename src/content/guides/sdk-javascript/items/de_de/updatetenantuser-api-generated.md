## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateTenantUserBody | UpdateTenantUserBody | Ja |  |
| updateComments | string | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'updateTenantUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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