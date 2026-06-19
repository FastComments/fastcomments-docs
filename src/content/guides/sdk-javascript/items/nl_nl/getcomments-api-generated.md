## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | number | Nee |  |
| limit | number | Nee |  |
| skip | number | Nee |  |
| asTree | boolean | Nee |  |
| skipChildren | number | Nee |  |
| limitChildren | number | Nee |  |
| maxTreeDepth | number | Nee |  |
| urlId | string | Nee |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |
| contextUserId | string | Nee |  |
| hashTag | string | Nee |  |
| parentId | string | Nee |  |
| direction | SortDirections | Nee |  |
| fromDate | number | Nee |  |
| toDate | number | Nee |  |

## Response

Retourneert: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getComments Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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