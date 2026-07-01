## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| value | string | Nee |  |
| tenantId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`GetSearchPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchPagesResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getSearchPages Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const query: string = "network outage";
  const tenantId: string = "tenant-9876";
  const ssoToken: string = "sso-abc123def456";

  const searchResult: GetSearchPagesResponse = await getSearchPages(query, tenantId, ssoToken);
  const searchResultNoSso: GetSearchPagesResponse = await getSearchPages(query, tenantId);
})();
[inline-code-end]