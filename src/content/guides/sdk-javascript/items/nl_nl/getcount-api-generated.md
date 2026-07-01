## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| textSearch | string | Nee |  |
| byIPFromComment | string | Nee |  |
| filter | string | Nee |  |
| searchFilters | string | Nee |  |
| demo | boolean | Nee |  |
| tenantId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`GetCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCountResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getCount Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function main(): Promise<void> {
  const count: GetCountResponse = await getCount({
    textSearch: "order issue",
    byIPFromComment: "198.51.100.23",
    filter: "pending",
    demo: true,
    tenantId: "acme_corp",
    sso: "sso_abcdef123456"
  });
  console.log(count);
}
main();
[inline-code-end]