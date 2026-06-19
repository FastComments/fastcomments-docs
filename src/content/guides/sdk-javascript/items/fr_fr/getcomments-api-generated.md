## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| page | number | Non |  |
| limit | number | Non |  |
| skip | number | Non |  |
| asTree | boolean | Non |  |
| skipChildren | number | Non |  |
| limitChildren | number | Non |  |
| maxTreeDepth | number | Non |  |
| urlId | string | Non |  |
| userId | string | Non |  |
| anonUserId | string | Non |  |
| contextUserId | string | Non |  |
| hashTag | string | Non |  |
| parentId | string | Non |  |
| direction | SortDirections | Non |  |
| fromDate | number | Non |  |
| toDate | number | Non |  |

## Réponse

Renvoie: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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