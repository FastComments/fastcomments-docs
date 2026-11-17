## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | Sí |  |
| sso | string | No |  |
| includeUserInfo | PageReactIncludeUserInfo | No |  |

## Respuesta

Devuelve: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getV2PageReacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc1234567890';
const urlId: string = '/blog/my-article';

const result: GetV2PageReacts = await getV2PageReacts(tenantId, urlId, 'sso-token-value', 'basic');
[inline-code-end]