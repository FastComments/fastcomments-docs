## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| page | number | לא |  |
| limit | number | לא |  |
| skip | number | לא |  |
| asTree | boolean | לא |  |
| skipChildren | number | לא |  |
| limitChildren | number | לא |  |
| maxTreeDepth | number | לא |  |
| urlId | string | לא |  |
| userId | string | לא |  |
| anonUserId | string | לא |  |
| contextUserId | string | לא |  |
| hashTag | string | לא |  |
| parentId | string | לא |  |
| direction | SortDirections | לא |  |
| fromDate | number | לא |  |
| toDate | number | לא |  |

## תשובה

מחזיר: [`GetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const page: number = 2;
const limit: number = 50;
const asTree: boolean = true;
const urlId: string = "article_5678";
const direction: SortDirections = "desc";
const fromDate: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // לפני שבוע
const toDate: number = Date.now();

const commentsResponse: GetCommentsResponse = await getComments({
  tenantId,
  page,
  limit,
  asTree,
  urlId,
  direction,
  fromDate,
  toDate,
});
[inline-code-end]