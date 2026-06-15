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

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-news';
const urlId: string = '/articles/2026/fastcomments-update';
const page: number = 1;
const skip: number = 0;
const limit: number = 25;
const countChildren: boolean = true;
const includeConfig: boolean = true;
const result: GetCommentsPublic200Response = await getCommentsPublic(
  tenantId,
  urlId,
  page,
  undefined,
  undefined,
  skip,
  undefined,
  limit,
  undefined,
  countChildren,
  undefined,
  includeConfig
);
[inline-code-end]

---