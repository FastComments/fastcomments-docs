## Parámetros

| Name | Type | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| page | number | No |  |
| limit | number | No |  |
| skip | number | No |  |
| asTree | boolean | No |  |
| skipChildren | number | No |  |
| limitChildren | number | No |  |
| maxTreeDepth | number | No |  |
| urlId | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |
| contextUserId | string | No |  |
| hashTag | string | No |  |
| parentId | string | No |  |
| direction | SortDirections | No |  |

## Respuesta

Devuelve: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // página
  20, // límite
  0, // omitir
  true, // como árbol
  1, // omitir hijos
  3, // límite de hijos
  4, // profundidad máxima del árbol
  'articles/2026/new-product-launch', // id de la URL
  'user_7890', // id de usuario
  'anon_4f3b2', // id de usuario anónimo
  undefined, // id de usuario en contexto
  '#launch', // etiqueta hash
  undefined // id del comentario padre
);
[inline-code-end]