## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| locale | string | Нет |  |
| rating | string | Нет |  |
| page | number | Нет |  |

## Response

Возвращает: [`GetGifsTrending200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrending200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример использования getGifsTrending'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const tenantId: string = 'tenant_8b3f2c';
  const locale: string = 'en-US';
  const rating: string = 'pg';
  const page: number = 1;
  const result: GetGifsTrending200Response = await getGifsTrending(tenantId, locale, rating, page);
  console.log(result);
}
main();
[inline-code-end]