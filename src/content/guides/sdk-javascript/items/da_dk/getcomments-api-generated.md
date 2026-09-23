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

## Svar

Returnerer: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getComments Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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