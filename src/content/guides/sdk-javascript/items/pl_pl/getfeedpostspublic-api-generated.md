req
tenantId
afterId

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| afterId | string | Nie |  |
| limit | number | Nie |  |
| tags | Array<string> | Nie |  |
| sso | string | Nie |  |
| isCrawler | boolean | Nie |  |
| includeUserInfo | boolean | Nie |  |

## Odpowiedź

Zwraca: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicFeedPostsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'getFeedPostsPublic Przykład'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const afterId: string = "post-20230915-abc123";
const limit: number = 15;
const tags: string[] = ["technology", "innovation"];
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiIxMjM0NSJ9.s3cr3tSignature";
const isCrawler: boolean = false;
const includeUserInfo: boolean = true;

const feedResponse: PublicFeedPostsResponse = await getFeedPostsPublic(
  tenantId,
  afterId,
  limit,
  tags,
  sso,
  isCrawler,
  includeUserInfo
);
[inline-code-end]