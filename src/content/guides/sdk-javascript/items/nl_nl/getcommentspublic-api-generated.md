req
tenantId
urlId

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nee |  |
| direction | SortDirections | Nee |  |
| sso | string | Nee |  |
| skip | number | Nee |  |
| skipChildren | number | Nee |  |
| limit | number | Nee |  |
| limitChildren | number | Nee |  |
| countChildren | boolean | Nee |  |
| fetchPageForCommentId | string | Nee |  |
| includeConfig | boolean | Nee |  |
| countAll | boolean | Nee |  |
| includei10n | boolean | Nee |  |
| locale | string | Nee |  |
| modules | string | Nee |  |
| isCrawler | boolean | Nee |  |
| includeNotificationCount | boolean | Nee |  |
| asTree | boolean | Nee |  |
| maxTreeDepth | number | Nee |  |
| useFullTranslationIds | boolean | Nee |  |
| parentId | string | Nee |  |
| searchText | string | Nee |  |
| hashTags | Array<string> | Nee |  |
| userId | string | Nee |  |
| customConfigStr | string | Nee |  |
| afterCommentId | string | Nee |  |
| beforeCommentId | string | Nee |  |

## Respons

Retourneert: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)