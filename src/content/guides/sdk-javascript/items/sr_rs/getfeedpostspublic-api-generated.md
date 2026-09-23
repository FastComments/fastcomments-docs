req
tenantId
afterId

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| afterId | string | Ne |  |
| limit | number | Ne |  |
| tags | Array<string> | Ne |  |
| sso | string | Ne |  |
| isCrawler | boolean | Ne |  |
| includeUserInfo | boolean | Ne |  |

## Odgovor

Vraća: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicFeedPostsResponse.ts)

## Primer

[inline-code-attrs-start title = 'getFeedPostsPublic Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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