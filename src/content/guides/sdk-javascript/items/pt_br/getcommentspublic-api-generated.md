req
tenantId
urlId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
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

## Resposta

Retorna: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

---