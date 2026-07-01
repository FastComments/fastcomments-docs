## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| search | string | Yes |  |
| locale | string | No |  |
| rating | string | No |  |
| page | number | No |  |

## Відповідь

Повертає: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsSearchResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getGifsSearch'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_9f8b7c";
  const search: string = "funny cats";
  const locale: string = "en-US";
  const rating: string = "pg";
  const page: number = 1;

  const result: GetGifsSearchResponse = await getGifsSearch(
    tenantId,
    search,
    locale,
    rating,
    page
  );

  console.log(result);
}

demo();
[inline-code-end]