## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| page | number | 아니오 |  |
| limit | number | 아니오 |  |
| skip | number | 아니오 |  |
| asTree | boolean | 아니오 |  |
| skipChildren | number | 아니오 |  |
| limitChildren | number | 아니오 |  |
| maxTreeDepth | number | 아니오 |  |
| urlId | string | 아니오 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |
| contextUserId | string | 아니오 |  |
| hashTag | string | 아니오 |  |
| parentId | string | 아니오 |  |
| direction | SortDirections | 아니오 |  |
| fromDate | number | 아니오 |  |
| toDate | number | 아니오 |  |

## 응답

반환: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getComments 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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