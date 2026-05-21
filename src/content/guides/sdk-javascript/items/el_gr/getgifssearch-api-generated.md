---
## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| search | string | Ναι |  |
| locale | string | Όχι |  |
| rating | string | Όχι |  |
| page | number | Όχι |  |

## Απόκριση

Επιστρέφει: [`GifSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GifSearchResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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