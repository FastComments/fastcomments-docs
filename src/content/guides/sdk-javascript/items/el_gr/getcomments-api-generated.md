## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| page | number | Όχι |  |
| limit | number | Όχι |  |
| skip | number | Όχι |  |
| asTree | boolean | Όχι |  |
| skipChildren | number | Όχι |  |
| limitChildren | number | Όχι |  |
| maxTreeDepth | number | Όχι |  |
| urlId | string | Όχι |  |
| userId | string | Όχι |  |
| anonUserId | string | Όχι |  |
| contextUserId | string | Όχι |  |
| hashTag | string | Όχι |  |
| parentId | string | Όχι |  |
| direction | SortDirections | Όχι |  |
| fromDate | number | Όχι |  |
| toDate | number | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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