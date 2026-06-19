## Параметри

| Име | Тип | Потребно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| search | string | Да |  |
| locale | string | Не |  |
| rating | string | Не |  |
| page | number | Не |  |

## Одговор

Враћа: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## Пример

[inline-code-attrs-start title = 'getGifsSearch Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const search: string = 'golden hour sunset';
const locale: string = 'en-US';
const rating: string = 'pg';
const page: number = 1;
const result: GetGifsSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---