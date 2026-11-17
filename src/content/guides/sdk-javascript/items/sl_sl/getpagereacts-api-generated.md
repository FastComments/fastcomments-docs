Prilagojeni števci odzivov react za stran, po možnosti skupaj z uporabniki, ki so se odzvali.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| includeUserInfo | PageReactIncludeUserInfo | Ne |  |

## Odgovor

Vrne: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageReactsAPIResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getPageReacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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