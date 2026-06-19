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

## תגובה

מחזיר: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_789";
const page: number = 1;
const limit: number = 25;
const asTree: boolean = true;
const maxTreeDepth: number = 3;
const urlId: string = "articles/2026/fastcomments-intro";
const userId: string = "user_12345";
const direction: SortDirections = "desc";
const fromDate: number = 1672531200000;
const toDate: number = Date.now();

const result: APIGetCommentsResponse = await getComments(
  tenantId,
  page,
  limit,
  0,
  asTree,
  0,
  5,
  maxTreeDepth,
  urlId,
  userId,
  undefined,
  undefined,
  "#release",
  undefined,
  direction,
  fromDate,
  toDate
);
[inline-code-end]

---