## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| createModeratorBody | CreateModeratorBody | Sí |  |

## Respuesta

Devuelve: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateModeratorResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_83f4b7a2';
const createModeratorBody: CreateModeratorBody = {
  email: 'renee.alvarez@acme-corp.com',
  fullName: 'Renee Alvarez',
  roles: ['content_moderator'],
  notify: true // parámetro opcional demostrado
};
const result: CreateModeratorResponse = await createModerator(tenantId, createModeratorBody);
[inline-code-end]