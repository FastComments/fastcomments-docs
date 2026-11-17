req
tenantId
urlId

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| page | number | いいえ |  |
| direction | SortDirections | いいえ |  |
| sso | string | いいえ |  |
| skip | number | いいえ |  |
| skipChildren | number | いいえ |  |
| limit | number | いいえ |  |
| limitChildren | number | いいえ |  |
| countChildren | boolean | いいえ |  |
| fetchPageForCommentId | string | いいえ |  |
| includeConfig | boolean | いいえ |  |
| countAll | boolean | いいえ |  |
| includei10n | boolean | いいえ |  |
| locale | string | いいえ |  |
| modules | string | いいえ |  |
| isCrawler | boolean | いいえ |  |
| includeNotificationCount | boolean | いいえ |  |
| asTree | boolean | いいえ |  |
| maxTreeDepth | number | いいえ |  |
| useFullTranslationIds | boolean | いいえ |  |
| parentId | string | いいえ |  |
| searchText | string | いいえ |  |
| hashTags | Array<string> | いいえ |  |
| userId | string | いいえ |  |
| customConfigStr | string | いいえ |  |
| afterCommentId | string | いいえ |  |
| beforeCommentId | string | いいえ |  |
| questionId | string | いいえ |  |
| questionValue | number | いいえ |  |

## レスポンス

戻り値: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## 例

[inline-code-attrs-start title = 'getCommentsPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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