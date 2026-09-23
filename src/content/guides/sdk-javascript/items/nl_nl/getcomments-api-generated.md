## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
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

## Respons

Retourneert: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getComments Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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