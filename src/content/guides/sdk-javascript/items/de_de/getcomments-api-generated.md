## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| page | number | Nein |  |
| limit | number | Nein |  |
| skip | number | Nein |  |
| asTree | boolean | Nein |  |
| skipChildren | number | Nein |  |
| limitChildren | number | Nein |  |
| maxTreeDepth | number | Nein |  |
| urlId | string | Nein |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |
| contextUserId | string | Nein |  |
| hashTag | string | Nein |  |
| parentId | string | Nein |  |
| direction | SortDirections | Nein |  |
| fromDate | number | Nein |  |
| toDate | number | Nein |  |

## Antwort

Rückgabe: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getComments Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments(): Promise<void> {
  const tenantId: string = "acme-corp";
  const page: number = 1;
  const limit: number = 50;
  const asTree: boolean = false;
  const direction: SortDirections = "asc";
  const fromDate: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 days ago
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