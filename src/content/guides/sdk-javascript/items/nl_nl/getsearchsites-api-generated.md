## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| value | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSiteSearchResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getSearchSites'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runSearches(): Promise<void> {
  const tenantId: string = "tenant-987654";
  const query: string = "offensive content";
  const ssoToken: string = "sso-token-xyz";

  const fullResult: ModerationSiteSearchResponse = await getSearchSites(tenantId, query, ssoToken);
  const minimalResult: ModerationSiteSearchResponse = await getSearchSites(tenantId);
}

runSearches();
[inline-code-end]