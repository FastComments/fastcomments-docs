## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| locale | string | Не |  |
| rating | string | Не |  |
| page | number | Не |  |

## Отговор

Връща: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrendingResponse.ts)

## Пример

[inline-code-attrs-start title = 'getGifsTrending Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-media';
const locale: string = 'en-US';
const rating: string = 'pg-13';
const page: number = 2;
const trending: GetGifsTrendingResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---