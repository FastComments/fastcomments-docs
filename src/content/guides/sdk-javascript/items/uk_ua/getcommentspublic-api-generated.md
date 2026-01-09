---
req
tenantId
urlId

## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Так |  |
| page | number | Ні |  |
| direction | SortDirections | Ні |  |
| sso | string | Ні |  |
| skip | number | Ні |  |
| skipChildren | number | Ні |  |
| limit | number | Ні |  |
| limitChildren | number | Ні |  |
| countChildren | boolean | Ні |  |
| fetchPageForCommentId | string | Ні |  |
| includeConfig | boolean | Ні |  |
| countAll | boolean | Ні |  |
| includei10n | boolean | Ні |  |
| locale | string | Ні |  |
| modules | string | Ні |  |
| isCrawler | boolean | Ні |  |
| includeNotificationCount | boolean | Ні |  |
| asTree | boolean | Ні |  |
| maxTreeDepth | number | Ні |  |
| useFullTranslationIds | boolean | Ні |  |
| parentId | string | Ні |  |
| searchText | string | Ні |  |
| hashTags | Array<string> | Ні |  |
| userId | string | Ні |  |
| customConfigStr | string | Ні |  |
| afterCommentId | string | Ні |  |
| beforeCommentId | string | Ні |  |

## Відповідь

Повертає: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

---