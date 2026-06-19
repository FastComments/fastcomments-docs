## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| page | number | 否 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |
| asTree | boolean | 否 |  |
| skipChildren | number | 否 |  |
| limitChildren | number | 否 |  |
| maxTreeDepth | number | 否 |  |
| urlId | string | 否 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |
| contextUserId | string | 否 |  |
| hashTag | string | 否 |  |
| parentId | string | 否 |  |
| direction | SortDirections | 否 |  |
| fromDate | number | 否 |  |
| toDate | number | 否 |  |

## 响应

返回: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getComments 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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