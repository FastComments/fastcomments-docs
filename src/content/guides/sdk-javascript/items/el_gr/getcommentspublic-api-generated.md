req
tenantId
urlId

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| page | number | Όχι |  |
| direction | SortDirections | Όχι |  |
| sso | string | Όχι |  |
| skip | number | Όχι |  |
| skipChildren | number | Όχι |  |
| limit | number | Όχι |  |
| limitChildren | number | Όχι |  |
| countChildren | boolean | Όχι |  |
| fetchPageForCommentId | string | Όχι |  |
| includeConfig | boolean | Όχι |  |
| countAll | boolean | Όχι |  |
| includei10n | boolean | Όχι |  |
| locale | string | Όχι |  |
| modules | string | Όχι |  |
| isCrawler | boolean | Όχι |  |
| includeNotificationCount | boolean | Όχι |  |
| asTree | boolean | Όχι |  |
| maxTreeDepth | number | Όχι |  |
| useFullTranslationIds | boolean | Όχι |  |
| parentId | string | Όχι |  |
| searchText | string | Όχι |  |
| hashTags | Array<string> | Όχι |  |
| userId | string | Όχι |  |
| customConfigStr | string | Όχι |  |
| afterCommentId | string | Όχι |  |
| beforeCommentId | string | Όχι |  |
| questionId | string | Όχι |  |
| questionValue | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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