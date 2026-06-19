## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| page | number | Hayır |  |
| limit | number | Hayır |  |
| skip | number | Hayır |  |
| asTree | boolean | Hayır |  |
| skipChildren | number | Hayır |  |
| limitChildren | number | Hayır |  |
| maxTreeDepth | number | Hayır |  |
| urlId | string | Hayır |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |
| contextUserId | string | Hayır |  |
| hashTag | string | Hayır |  |
| parentId | string | Hayır |  |
| direction | SortDirections | Hayır |  |
| fromDate | number | Hayır |  |
| toDate | number | Hayır |  |

## Yanıt

Dönüş: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getComments Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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