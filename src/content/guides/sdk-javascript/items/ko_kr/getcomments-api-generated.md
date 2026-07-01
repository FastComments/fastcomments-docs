## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|--------|------|
| tenantId | string | 예 |  |
| page | number | 아니오 |  |
| limit | number | 아니오 |  |
| skip | number | 아니오 |  |
| asTree | boolean | 아니오 |  |
| skipChildren | number | 아니오 |  |
| limitChildren | number | 아니오 |  |
| maxTreeDepth | number | 아니오 |  |
| urlId | string | 아니오 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |
| contextUserId | string | 아니오 |  |
| hashTag | string | 아니오 |  |
| parentId | string | 아니오 |  |
| direction | SortDirections | 아니오 |  |
| fromDate | number | 아니오 |  |
| toDate | number | 아니오 |  |

## 응답

반환: [`GetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsResponse.ts)

## 예시

[inline-code-attrs-start title = 'getComments 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const page: number = 2;
const limit: number = 50;
const asTree: boolean = true;
const urlId: string = "article_5678";
const direction: SortDirections = "desc";
const fromDate: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // 일주일 전
const toDate: number = Date.now();

const commentsResponse: GetCommentsResponse = await getComments({
  tenantId,
  page,
  limit,
  asTree,
  urlId,
  direction,
  fromDate,
  toDate,
});
[inline-code-end]