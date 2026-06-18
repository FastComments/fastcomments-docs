req
tenantId
urlId

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| page | number | 아니요 |  |
| direction | SortDirections | 아니요 |  |
| sso | string | 아니요 |  |
| skip | number | 아니요 |  |
| skipChildren | number | 아니요 |  |
| limit | number | 아니요 |  |
| limitChildren | number | 아니요 |  |
| countChildren | boolean | 아니요 |  |
| fetchPageForCommentId | string | 아니요 |  |
| includeConfig | boolean | 아니요 |  |
| countAll | boolean | 아니요 |  |
| includei10n | boolean | 아니요 |  |
| locale | string | 아니요 |  |
| modules | string | 아니요 |  |
| isCrawler | boolean | 아니요 |  |
| includeNotificationCount | boolean | 아니요 |  |
| asTree | boolean | 아니요 |  |
| maxTreeDepth | number | 아니요 |  |
| useFullTranslationIds | boolean | 아니요 |  |
| parentId | string | 아니요 |  |
| searchText | string | 아니요 |  |
| hashTags | Array<string> | 아니요 |  |
| userId | string | 아니요 |  |
| customConfigStr | string | 아니요 |  |
| afterCommentId | string | 아니요 |  |
| beforeCommentId | string | 아니요 |  |

## 응답

반환: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'getCommentsPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-news';
const urlId: string = '/articles/2026/fastcomments-update';
const page: number = 1;
const skip: number = 0;
const limit: number = 25;
const countChildren: boolean = true;
const includeConfig: boolean = true;
const result: GetCommentsPublic200Response = await getCommentsPublic(
  tenantId,
  urlId,
  page,
  undefined,
  undefined,
  skip,
  undefined,
  limit,
  undefined,
  countChildren,
  undefined,
  includeConfig
);
[inline-code-end]

---