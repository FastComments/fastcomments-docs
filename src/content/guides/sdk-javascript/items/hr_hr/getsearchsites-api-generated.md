## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| value | string | No |  |
| sso | string | No |  |

## Odgovor

Vraća: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSiteSearchResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getSearchSites'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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