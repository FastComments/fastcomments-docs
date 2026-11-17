Aangepaste react-tellingen voor een pagina, optioneel met de gebruikers die hebben gereageerd.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| includeUserInfo | PageReactIncludeUserInfo | Nee |  |

## Response

Geeft terug: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageReactsAPIResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getPageReacts Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'my-blog-post-42';
const includeUserInfo: PageReactIncludeUserInfo = true;

const response: GetPageReactsAPIResponse = await getPageReacts(
  tenantId,
  urlId,
  includeUserInfo
);

const reacts = response.pageReacts;
const reactCount: number = reacts.like.count;
const users: PageCustomReactUser[] = reacts.like.userDetails ?? [];
[inline-code-end]