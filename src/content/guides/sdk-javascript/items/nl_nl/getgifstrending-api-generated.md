## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| locale | string | Nee |  |
| rating | string | Nee |  |
| page | number | Nee |  |

## Respons

Geeft terug: [`GetGifsTrending200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrending200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getGifsTrending Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---