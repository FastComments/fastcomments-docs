## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| id | string | Sí |  |
| title | string | No |  |

## Respuesta

Devuelve: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createV2PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_91f4b3b';
const urlId: string = 'https://news.site.com/articles/2026/06/fastcomments-integration';
const id: string = 'react_5f2c1a';
const title: string = 'FastComments Integration — June 2026';

const reactionWithTitle: CreateV1PageReact = await createV2PageReact(tenantId, urlId, id, title);
const reactionWithoutTitle: CreateV1PageReact = await createV2PageReact(tenantId, urlId, id);
[inline-code-end]

---