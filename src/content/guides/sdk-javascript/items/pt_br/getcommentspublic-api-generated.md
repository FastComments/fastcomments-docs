req
tenantId
urlId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| page | number | Não |  |
| direction | SortDirections | Não |  |
| sso | string | Não |  |
| skip | number | Não |  |
| skipChildren | number | Não |  |
| limit | number | Não |  |
| limitChildren | number | Não |  |
| countChildren | boolean | Não |  |
| fetchPageForCommentId | string | Não |  |
| includeConfig | boolean | Não |  |
| countAll | boolean | Não |  |
| includei10n | boolean | Não |  |
| locale | string | Não |  |
| modules | string | Não |  |
| isCrawler | boolean | Não |  |
| includeNotificationCount | boolean | Não |  |
| asTree | boolean | Não |  |
| maxTreeDepth | number | Não |  |
| useFullTranslationIds | boolean | Não |  |
| parentId | string | Não |  |
| searchText | string | Não |  |
| hashTags | Array<string> | Não |  |
| userId | string | Não |  |
| customConfigStr | string | Não |  |
| afterCommentId | string | Não |  |
| beforeCommentId | string | Não |  |
| questionId | string | Não |  |
| questionValue | number | Não |  |

## Resposta

Retorna: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponseWithPresencePublicComment.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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