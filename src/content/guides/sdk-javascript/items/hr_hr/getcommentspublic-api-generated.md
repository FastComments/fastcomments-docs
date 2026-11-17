req
tenantId
urlId

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| page | number | Ne |  |
| direction | SortDirections | Ne |  |
| sso | string | Ne |  |
| skip | number | Ne |  |
| skipChildren | number | Ne |  |
| limit | number | Ne |  |
| limitChildren | number | Ne |  |
| countChildren | boolean | Ne |  |
| fetchPageForCommentId | string | Ne |  |
| includeConfig | boolean | Ne |  |
| countAll | boolean | Ne |  |
| includei10n | boolean | Ne |  |
| locale | string | Ne |  |
| modules | string | Ne |  |
| isCrawler | boolean | Ne |  |
| includeNotificationCount | boolean | Ne |  |
| asTree | boolean | Ne |  |
| maxTreeDepth | number | Ne |  |
| useFullTranslationIds | boolean | Ne |  |
| parentId | string | Ne |  |
| searchText | string | Ne |  |
| hashTags | Array<string> | Ne |  |
| userId | string | Ne |  |
| customConfigStr | string | Ne |  |
| afterCommentId | string | Ne |  |
| beforeCommentId | string | Ne |  |
| questionId | string | Ne |  |
| questionValue | number | Ne |  |

## Odgovor

Vraća: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer za getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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