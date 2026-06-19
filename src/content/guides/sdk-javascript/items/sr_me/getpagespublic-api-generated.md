Листа страница за тенанта. Користи се од стране FChat десктоп клијента за попуњавање његове листе соба. Потребно је да је `enableFChat` постављен на true у коначном прилагођеном конфигу за сваку страницу. Странице које захтијевају SSO се филтрирају према приступу група корисника који шаље захтјев.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---