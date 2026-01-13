req
tenantId
urlId

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Ναι |  |
| page | number | Όχι |  |
| direction | SortDirections | Όχι |  |
| sso | string | Όχι |  |
| skip | number | Όχι |  |
| skipChildren | number | Όχι |  |
| limit | number | Όχι |  |
| limitChildren | number | Όχι |  |
| countChildren | boolean | Όχι |  |
| fetchPageForCommentId | string | Όχι |  |
| includeConfig | boolean | Όχι |  |
| countAll | boolean | Όχι |  |
| includei10n | boolean | Όχι |  |
| locale | string | Όχι |  |
| modules | string | Όχι |  |
| isCrawler | boolean | Όχι |  |
| includeNotificationCount | boolean | Όχι |  |
| asTree | boolean | Όχι |  |
| maxTreeDepth | number | Όχι |  |
| useFullTranslationIds | boolean | Όχι |  |
| parentId | string | Όχι |  |
| searchText | string | Όχι |  |
| hashTags | Array<string> | Όχι |  |
| userId | string | Όχι |  |
| customConfigStr | string | Όχι |  |
| afterCommentId | string | Όχι |  |
| beforeCommentId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)