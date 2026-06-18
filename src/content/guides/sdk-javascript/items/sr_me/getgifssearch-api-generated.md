## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| search | string | Да |  |
| locale | string | Не |  |
| rating | string | Не |  |
| page | number | Не |  |

## Одговор

Враћа: [`GetGifsSearch200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearch200Response.ts)

## Пример

[inline-code-attrs-start title = 'getGifsSearch пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_9876";
const search: string = "cat playing piano";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 1;
const result: GetGifsSearch200Response = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---