## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Yes |  |
| updateComments | string | No |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład replaceTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acmeCorp";
const id: string = "user_84b2";
const replaceTenantUserBody: ReplaceTenantUserBody = {
  email: "alice.jenkins@acmecorp.com",
  displayName: "Alice Jenkins",
  roles: ["moderator", "editor"],
  disabled: false
} as ReplaceTenantUserBody;
const updateComments: string = "Migrated user account and reattributed historical comments";

const result: FlagCommentPublic200Response = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]

---