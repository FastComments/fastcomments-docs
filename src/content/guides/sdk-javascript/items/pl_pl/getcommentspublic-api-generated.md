req
tenantId
urlId

## Parametry

| Nazwa | Typ | Wymagane | Opis |
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

## Odpowiedź

Zwraca: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)