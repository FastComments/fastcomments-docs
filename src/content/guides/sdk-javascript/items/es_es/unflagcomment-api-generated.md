---
## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | Sí |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Respuesta

Devuelve: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de unFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const id: string = 'cmt-9b8f7d6a5';
const userId: string = 'user-42a7c9e1';

const result: FlagCommentResponse = await unFlagComment(tenantId, id, userId);
[inline-code-end]

---