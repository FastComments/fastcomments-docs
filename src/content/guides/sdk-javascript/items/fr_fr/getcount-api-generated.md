## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| textSearch | string | Non |  |
| byIPFromComment | string | Non |  |
| filter | string | Non |  |
| searchFilters | string | Non |  |
| demo | boolean | Non |  |
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`GetCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCountResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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