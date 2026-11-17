req
tenantId
urlId

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| page | number | Hayır |  |
| direction | SortDirections | Hayır |  |
| sso | string | Hayır |  |
| skip | number | Hayır |  |
| skipChildren | number | Hayır |  |
| limit | number | Hayır |  |
| limitChildren | number | Hayır |  |
| countChildren | boolean | Hayır |  |
| fetchPageForCommentId | string | Hayır |  |
| includeConfig | boolean | Hayır |  |
| countAll | boolean | Hayır |  |
| includei10n | boolean | Hayır |  |
| locale | string | Hayır |  |
| modules | string | Hayır |  |
| isCrawler | boolean | Hayır |  |
| includeNotificationCount | boolean | Hayır |  |
| asTree | boolean | Hayır |  |
| maxTreeDepth | number | Hayır |  |
| useFullTranslationIds | boolean | Hayır |  |
| parentId | string | Hayır |  |
| searchText | string | Hayır |  |
| hashTags | Array<string> | Hayır |  |
| userId | string | Hayır |  |
| customConfigStr | string | Hayır |  |
| afterCommentId | string | Hayır |  |
| beforeCommentId | string | Hayır |  |
| questionId | string | Hayır |  |
| questionValue | number | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Örnek

[inline-code-attrs-start title = 'getCommentsPublic Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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