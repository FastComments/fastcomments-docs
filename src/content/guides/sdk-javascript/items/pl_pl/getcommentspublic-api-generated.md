req
tenantId
urlId

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| page | number | Nie |  |
| direction | SortDirections | Nie |  |
| sso | string | Nie |  |
| skip | number | Nie |  |
| skipChildren | number | Nie |  |
| limit | number | Nie |  |
| limitChildren | number | Nie |  |
| countChildren | boolean | Nie |  |
| fetchPageForCommentId | string | Nie |  |
| includeConfig | boolean | Nie |  |
| countAll | boolean | Nie |  |
| includei10n | boolean | Nie |  |
| locale | string | Nie |  |
| modules | string | Nie |  |
| isCrawler | boolean | Nie |  |
| includeNotificationCount | boolean | Nie |  |
| asTree | boolean | Nie |  |
| maxTreeDepth | number | Nie |  |
| useFullTranslationIds | boolean | Nie |  |
| parentId | string | Nie |  |
| searchText | string | Nie |  |
| hashTags | Array<string> | Nie |  |
| userId | string | Nie |  |
| customConfigStr | string | Nie |  |
| afterCommentId | string | Nie |  |
| beforeCommentId | string | Nie |  |
| questionId | string | Nie |  |
| questionValue | number | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
console.log(`Pobrano ${comments.length} komentarzy, ${presenceCount} użytkowników online`);
[inline-code-end]