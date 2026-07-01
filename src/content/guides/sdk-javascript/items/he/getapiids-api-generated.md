## Parameters

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| textSearch | string | לא |  |
| byIPFromComment | string | לא |  |
| filters | string | לא |  |
| searchFilters | string | לא |  |
| afterId | string | לא |  |
| demo | boolean | לא |  |
| tenantId | string | לא |  |
| sso | string | לא |  |

## Response

מחזירה: [`GetApiIdsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiIdsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getApiIds'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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