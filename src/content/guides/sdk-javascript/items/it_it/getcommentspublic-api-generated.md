req
tenantId
urlId

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| page | number | No |  |
| direction | SortDirections | No |  |
| sso | string | No |  |
| skip | number | No |  |
| skipChildren | number | No |  |
| limit | number | No |  |
| limitChildren | number | No |  |
| countChildren | boolean | No |  |
| fetchPageForCommentId | string | No |  |
| includeConfig | boolean | No |  |
| countAll | boolean | No |  |
| includei10n | boolean | No |  |
| locale | string | No |  |
| modules | string | No |  |
| isCrawler | boolean | No |  |
| includeNotificationCount | boolean | No |  |
| asTree | boolean | No |  |
| maxTreeDepth | number | No |  |
| useFullTranslationIds | boolean | No |  |
| parentId | string | No |  |
| searchText | string | No |  |
| hashTags | Array<string> | No |  |
| userId | string | No |  |
| customConfigStr | string | No |  |
| afterCommentId | string | No |  |
| beforeCommentId | string | No |  |

## Risposta

Restituisce: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)