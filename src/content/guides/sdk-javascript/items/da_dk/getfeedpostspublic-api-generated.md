req
tenantId
afterId

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| afterId | string | Nej |  |
| limit | number | Nej |  |
| tags | Array<string> | Nej |  |
| sso | string | Nej |  |
| isCrawler | boolean | Nej |  |
| includeUserInfo | boolean | Nej |  |

## Svar

Returnerer: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicFeedPostsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getFeedPostsPublic Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---