## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| search | string | Да |  |
| locale | string | Не |  |
| rating | string | Не |  |
| page | number | Не |  |

## Одговор

Враћа: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Пример

[inline-code-attrs-start title = 'getGifsSearch Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "global-media";
  const search: string = "laughing baby";
  const locale: string = "en-US";
  const rating: string = "pg";
  const page: number = 2;
  const result: GifSearchResponse = await getGifsSearch(tenantId, search, locale, rating, page);
  console.log(result);
})();
[inline-code-end]

---