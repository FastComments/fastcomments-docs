## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| textSearch | string | No |  |
| sso | string | No |  |

## Odgovor

Vraća: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSuggestResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getSearchSuggest'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-9f8b7c6d";
  const textSearch: string = "harassment";
  const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
  const suggestResponse: ModerationSuggestResponse = await getSearchSuggest(tenantId, textSearch, sso);
  const minimalResponse: ModerationSuggestResponse = await getSearchSuggest(tenantId);
})();
[inline-code-end]