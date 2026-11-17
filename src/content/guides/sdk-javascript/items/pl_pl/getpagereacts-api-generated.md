Niestandardowe liczniki reakcji dla strony, opcjonalnie z użytkownikami, którzy zareagowali.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| includeUserInfo | PageReactIncludeUserInfo | Nie |  |

## Odpowiedź

Zwraca: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPageReactsAPIResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getPageReacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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