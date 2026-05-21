## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| locale | string | Ні |  |
| rating | string | Ні |  |
| page | number | Ні |  |

## Відповідь

Повертає: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getGifsTrending'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42';
const locale: string = 'en-US';
const rating: string = 'PG';
const page: number = 1;
const result: GifSearchResponse = await getGifsTrending(tenantId, locale, rating, page);
[inline-code-end]

---