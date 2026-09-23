req
tenantId
afterId

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| afterId | string | いいえ |  |
| limit | number | いいえ |  |
| tags | Array<string> | いいえ |  |
| sso | string | いいえ |  |
| isCrawler | boolean | いいえ |  |
| includeUserInfo | boolean | いいえ |  |

## レスポンス

返却: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PublicFeedPostsResponse.ts)

## 例

[inline-code-attrs-start title = 'getFeedPostsPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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