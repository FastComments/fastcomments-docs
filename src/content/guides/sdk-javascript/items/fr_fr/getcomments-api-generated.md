## Paramètres

| Nom | Type | Obligatoire | Description |
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

Retourne : [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments(): Promise<void> {
  const tenantId: string = "acme-corp";
  const page: number = 1;
  const limit: number = 50;
  const asTree: boolean = false;
  const direction: SortDirections = "asc";
  const fromDate: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // il y a 30 jours
  const toDate: number = Date.now();

  const commentsResponse: APIGetCommentsResponse = await getComments(
    tenantId,
    page,
    limit,
    undefined,
    asTree,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    undefined,
    direction,
    fromDate,
    toDate
  );

  console.log(commentsResponse);
}
[inline-code-end]