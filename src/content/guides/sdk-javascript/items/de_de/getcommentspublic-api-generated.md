req
tenantId
urlId

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nein |  |
| direction | SortDirections | Nein |  |
| sso | string | Nein |  |
| skip | number | Nein |  |
| skipChildren | number | Nein |  |
| limit | number | Nein |  |
| limitChildren | number | Nein |  |
| countChildren | boolean | Nein |  |
| fetchPageForCommentId | string | Nein |  |
| includeConfig | boolean | Nein |  |
| countAll | boolean | Nein |  |
| includei10n | boolean | Nein |  |
| locale | string | Nein |  |
| modules | string | Nein |  |
| isCrawler | boolean | Nein |  |
| includeNotificationCount | boolean | Nein |  |
| asTree | boolean | Nein |  |
| maxTreeDepth | number | Nein |  |
| useFullTranslationIds | boolean | Nein |  |
| parentId | string | Nein |  |
| searchText | string | Nein |  |
| hashTags | Array<string> | Nein |  |
| userId | string | Nein |  |
| customConfigStr | string | Nein |  |
| afterCommentId | string | Nein |  |
| beforeCommentId | string | Nein |  |
| questionId | string | Nein |  |
| questionValue | number | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Beispiel

[inline-code-attrs-start title = 'getCommentsPublic Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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