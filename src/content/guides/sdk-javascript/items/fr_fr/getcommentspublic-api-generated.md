req
tenantId
urlId

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| page | number | Non |  |
| direction | SortDirections | Non |  |
| sso | string | Non |  |
| skip | number | Non |  |
| skipChildren | number | Non |  |
| limit | number | Non |  |
| limitChildren | number | Non |  |
| countChildren | boolean | Non |  |
| fetchPageForCommentId | string | Non |  |
| includeConfig | boolean | Non |  |
| countAll | boolean | Non |  |
| includei10n | boolean | Non |  |
| locale | string | Non |  |
| modules | string | Non |  |
| isCrawler | boolean | Non |  |
| includeNotificationCount | boolean | Non |  |
| asTree | boolean | Non |  |
| maxTreeDepth | number | Non |  |
| useFullTranslationIds | boolean | Non |  |
| parentId | string | Non |  |
| searchText | string | Non |  |
| hashTags | Array<string> | Non |  |
| userId | string | Non |  |
| customConfigStr | string | Non |  |
| afterCommentId | string | Non |  |
| beforeCommentId | string | Non |  |
| questionId | string | Non |  |
| questionValue | number | Non |  |

## Réponse

Retourne : [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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