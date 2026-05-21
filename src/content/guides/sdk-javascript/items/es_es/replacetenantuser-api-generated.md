## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Sí |  |
| updateComments | string | No |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de replaceTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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