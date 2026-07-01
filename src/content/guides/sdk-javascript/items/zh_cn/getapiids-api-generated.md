## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| textSearch | string | 否 |  |
| byIPFromComment | string | 否 |  |
| filters | string | 否 |  |
| searchFilters | string | 否 |  |
| afterId | string | 否 |  |
| demo | boolean | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetApiIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiIdsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getApiIds 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const textSearch: string = "urgent feedback";
const byIPFromComment: string = "203.0.113.42";
const filters: string = "status:approved";
const afterId: string = "comment-789";
const demo: boolean = true;
const tenantId: string = "tenant-001";

const apiIds: GetApiIdsResponse = await getApiIds({
  textSearch,
  byIPFromComment,
  filters,
  afterId,
  demo,
  tenantId,
});
[inline-code-end]