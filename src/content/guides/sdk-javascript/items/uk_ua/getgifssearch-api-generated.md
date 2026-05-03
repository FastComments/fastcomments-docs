## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| search | string | Так |  |
| locale | string | Ні |  |
| rating | string | Ні |  |
| page | number | Ні |  |

## Відповідь

Повертає: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад використання getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_fcm_42";
const search: string = "funny golden retriever";
const locale: string = "en-US";
const rating: string = "pg";
const page: number = 2;
const result: GifSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
[inline-code-end]

---