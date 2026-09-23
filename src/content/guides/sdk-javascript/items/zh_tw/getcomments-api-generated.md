## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| page | number | 否 |  |
| limit | number | 否 |  |
| skip | number | 否 |  |
| asTree | boolean | 否 |  |
| skipChildren | number | 否 |  |
| limitChildren | number | 否 |  |
| maxTreeDepth | number | 否 |  |
| urlId | string | 否 |  |
| userId | string | 否 |  |
| anonUserId | string | 否 |  |
| contextUserId | string | 否 |  |
| hashTag | string | 否 |  |
| parentId | string | 否 |  |
| direction | SortDirections | 否 |  |
| fromDate | number | 否 |  |
| toDate | number | 否 |  |

## 回應

返回：[`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetCommentsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getComments 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments(): Promise<void> {
  const tenantId: string = "acme-corp";
  const page: number = 1;
  const limit: number = 50;
  const asTree: boolean = false;
  const direction: SortDirections = "asc";
  const fromDate: number = Date.now() - 30 * 24 * 60 * 60 * 1000; // 30 天前
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

---