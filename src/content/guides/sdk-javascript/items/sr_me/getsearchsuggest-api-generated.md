## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| textSearch | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Odgovor

Vraća: [`GetSearchSuggestResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchSuggestResponse.ts)

## Primer

[inline-code-attrs-start title = 'getSearchSuggest Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example(): Promise<void> {
    const term: string = "fastcomments api";
    const tenant: string = "tenant_001";
    const sso: string = "sso_9a8b7c";

    const fullResult: GetSearchSuggestResponse = await getSearchSuggest(term, tenant, sso);
    const partialResult: GetSearchSuggestResponse = await getSearchSuggest(term);
}

example();
[inline-code-end]