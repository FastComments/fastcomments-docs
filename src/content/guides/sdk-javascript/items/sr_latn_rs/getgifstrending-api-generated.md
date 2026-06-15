## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| locale | string | Ne |  |
| rating | string | Ne |  |
| page | number | Ne |  |

## Odgovor

Vraća: [`GetGifsTrending200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetGifsTrending200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getGifsTrending'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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