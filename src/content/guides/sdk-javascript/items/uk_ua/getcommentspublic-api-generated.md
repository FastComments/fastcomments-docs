req
tenantId
urlId

## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| page | number | Ні |  |
| direction | SortDirections | Ні |  |
| sso | string | Ні |  |
| skip | number | Ні |  |
| skipChildren | number | Ні |  |
| limit | number | Ні |  |
| limitChildren | number | Ні |  |
| countChildren | boolean | Ні |  |
| fetchPageForCommentId | string | Ні |  |
| includeConfig | boolean | Ні |  |
| countAll | boolean | Ні |  |
| includei10n | boolean | Ні |  |
| locale | string | Ні |  |
| modules | string | Ні |  |
| isCrawler | boolean | Ні |  |
| includeNotificationCount | boolean | Ні |  |
| asTree | boolean | Ні |  |
| maxTreeDepth | number | Ні |  |
| useFullTranslationIds | boolean | Ні |  |
| parentId | string | Ні |  |
| searchText | string | Ні |  |
| hashTags | Array<string> | Ні |  |
| userId | string | Ні |  |
| customConfigStr | string | Ні |  |
| afterCommentId | string | Ні |  |
| beforeCommentId | string | Ні |  |
| questionId | string | Ні |  |
| questionValue | number | Ні |  |

## Відповідь

Повертає: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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