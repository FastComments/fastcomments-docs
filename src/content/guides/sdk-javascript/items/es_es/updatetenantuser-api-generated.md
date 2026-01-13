## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| updateTenantUserBody | UpdateTenantUserBody | Sí |  |
| updateComments | string | No |  |

## Respuesta

Devuelve: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f3b2';
const id: string = 'user_7a9d1c';
const updateComments: string = 'Promoted to moderator and updated contact email';
const updateTenantUserBody: UpdateTenantUserBody = {
  email: 'jane.doe+mod@example.com',
  displayName: 'Jane D.',
  roles: ['moderator'],
  isBanned: false,
  metadata: { department: 'community' }
};
const result: FlagCommentPublic200Response = await updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
[inline-code-end]

---