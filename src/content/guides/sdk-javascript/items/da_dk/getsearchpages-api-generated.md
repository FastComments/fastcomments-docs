## Parametre

| Navn | Type | Obligatorisk | Beskrivelse |
|------|------|--------------|-------------|
| value | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Svar

Returnerer: [`GetSearchPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSearchPagesResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getSearchPages Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const query: string = "network outage";
  const tenantId: string = "tenant-9876";
  const ssoToken: string = "sso-abc123def456";

  const searchResult: GetSearchPagesResponse = await getSearchPages(query, tenantId, ssoToken);
  const searchResultNoSso: GetSearchPagesResponse = await getSearchPages(query, tenantId);
})();
[inline-code-end]