req
tenantId
urlId

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nej |  |
| direction | SortDirections | Nej |  |
| sso | string | Nej |  |
| skip | number | Nej |  |
| skipChildren | number | Nej |  |
| limit | number | Nej |  |
| limitChildren | number | Nej |  |
| countChildren | boolean | Nej |  |
| fetchPageForCommentId | string | Nej |  |
| includeConfig | boolean | Nej |  |
| countAll | boolean | Nej |  |
| includei10n | boolean | Nej |  |
| locale | string | Nej |  |
| modules | string | Nej |  |
| isCrawler | boolean | Nej |  |
| includeNotificationCount | boolean | Nej |  |
| asTree | boolean | Nej |  |
| maxTreeDepth | number | Nej |  |
| useFullTranslationIds | boolean | Nej |  |
| parentId | string | Nej |  |
| searchText | string | Nej |  |
| hashTags | Array<string> | Nej |  |
| userId | string | Nej |  |
| customConfigStr | string | Nej |  |
| afterCommentId | string | Nej |  |
| beforeCommentId | string | Nej |  |
| questionId | string | Nej |  |
| questionValue | number | Nej |  |

## Svar

Returnerer: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Eksempel

[inline-code-attrs-start title = 'getCommentsPublic Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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