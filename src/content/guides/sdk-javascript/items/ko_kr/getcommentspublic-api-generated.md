req
tenantId
urlId

## 매개변수

| 이름 | 타입 | 필수 여부 | 설명 |
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
| questionId | string | 아니요 |  |
| questionValue | number | 아니요 |  |

## 응답

반환값: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## 예제

[inline-code-attrs-start title = 'getCommentsPublic 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
console.log(`${comments.length}개의 댓글을 가져왔으며, ${presenceCount}명의 사용자가 온라인 상태입니다`);
[inline-code-end]