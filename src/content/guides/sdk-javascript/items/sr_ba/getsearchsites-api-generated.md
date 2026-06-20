---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| value | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationSiteSearchResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getSearchSites Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const searchValue: string = 'fastcomments.com';
const ssoToken: string = 'sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
const responseWithSSO: ModerationSiteSearchResponse = await getSearchSites(searchValue, ssoToken);
const responseWithoutSSO: ModerationSiteSearchResponse = await getSearchSites('news.fastcompany.com');
[inline-code-end]

---