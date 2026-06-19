Lista las páginas para un tenant. Usado por el cliente de escritorio FChat para poblar su lista de salas.
Requiere `enableFChat` to be true on the resolved custom config for each page.
Las páginas que requieren SSO se filtran en función del acceso de grupo del usuario solicitante.

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## Respuesta

Devuelve: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---