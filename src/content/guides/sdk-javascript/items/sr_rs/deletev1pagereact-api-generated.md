## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|--------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Одговор

Враћа: [`DeleteV1PageReactResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteV1PageReactResponse.ts)

## Пример

[inline-code-attrs-start title = 'deleteV1PageReact Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const urlId: string = "article-2024-06-01";

const response: DeleteV1PageReactResponse = await deleteV1PageReact(tenantId, urlId);
console.log(response);
[inline-code-end]