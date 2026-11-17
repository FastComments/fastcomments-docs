req
tenantId
urlId

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| page | number | 否 |  |
| direction | SortDirections | 否 |  |
| sso | string | 否 |  |
| skip | number | 否 |  |
| skipChildren | number | 否 |  |
| limit | number | 否 |  |
| limitChildren | number | 否 |  |
| countChildren | boolean | 否 |  |
| fetchPageForCommentId | string | 否 |  |
| includeConfig | boolean | 否 |  |
| countAll | boolean | 否 |  |
| includei10n | boolean | 否 |  |
| locale | string | 否 |  |
| modules | string | 否 |  |
| isCrawler | boolean | 否 |  |
| includeNotificationCount | boolean | 否 |  |
| asTree | boolean | 否 |  |
| maxTreeDepth | number | 否 |  |
| useFullTranslationIds | boolean | 否 |  |
| parentId | string | 否 |  |
| searchText | string | 否 |  |
| hashTags | Array<string> | 否 |  |
| userId | string | 否 |  |
| customConfigStr | string | 否 |  |
| afterCommentId | string | 否 |  |
| beforeCommentId | string | 否 |  |
| questionId | string | 否 |  |
| questionValue | number | 否 |  |

## 回應

回傳:[`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## 範例

[inline-code-attrs-start title = 'getCommentsPublic 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc123def456';
const urlId: string = 'https://example.com/blog/my-post';

const response: GetCommentsResponseWithPresencePublicComment = await getCommentsPublic(
  tenantId,
  urlId,
  1,
  'DESC' as SortDirections,
  undefined,
  0,
  undefined,
  25,
  undefined,
  true,
  undefined,
  undefined,
  true,
  true,
  false,
  'en-US',
  undefined,
  false,
  true,
  undefined,
  true,
  3,
  undefined,
  undefined,
  'root',
  'authentication',
  ['announcement', 'qna'],
  undefined,
  undefined,
  undefined,
  undefined,
  undefined
);

const comments: PublicComment[] = response.comments;
const presenceCount: number = response.presence?.userCount ?? 0;
console.log(`Fetched ${comments.length} comments, ${presenceCount} users online`);
[inline-code-end]