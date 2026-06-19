## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| page | number | Nie |  |
| limit | number | Nie |  |
| skip | number | Nie |  |
| asTree | boolean | Nie |  |
| skipChildren | number | Nie |  |
| limitChildren | number | Nie |  |
| maxTreeDepth | number | Nie |  |
| urlId | string | Nie |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |
| contextUserId | string | Nie |  |
| hashTag | string | Nie |  |
| parentId | string | Nie |  |
| direction | SortDirections | Nie |  |
| fromDate | number | Nie |  |
| toDate | number | Nie |  |

## Odpowiedź

Zwraca: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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