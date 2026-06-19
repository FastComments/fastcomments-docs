## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | number | Nej |  |
| limit | number | Nej |  |
| skip | number | Nej |  |
| asTree | boolean | Nej |  |
| skipChildren | number | Nej |  |
| limitChildren | number | Nej |  |
| maxTreeDepth | number | Nej |  |
| urlId | string | Nej |  |
| userId | string | Nej |  |
| anonUserId | string | Nej |  |
| contextUserId | string | Nej |  |
| hashTag | string | Nej |  |
| parentId | string | Nej |  |
| direction | SortDirections | Nej |  |
| fromDate | number | Nej |  |
| toDate | number | Nej |  |

## Respons

Returnerer: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getComments-eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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