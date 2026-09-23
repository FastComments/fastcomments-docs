---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| page | number | 否 |  |
| count | number | 否 |  |
| textSearch | string | 否 |  |
| byIPFromComment | string | 否 |  |
| filters | string | 否 |  |
| searchFilters | string | 否 |  |
| sorts | string | 否 |  |
| demo | boolean | 否 |  |
| sso | string | 否 |  |

## 回應

返回: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getApiComments 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const page: number = 2;
  const count: number = 20;
  const textSearch: string = "typescript";
  const byIPFromComment: string = "192.168.1.100";
  const filters: string = "spam,offensive";
  const searchFilters: string = "user:john";
  const sorts: string = "date_desc";
  const demo: boolean = true;
  const sso: string = "sso_token_abc";

  const response: ModerationAPIGetCommentsResponse = await getApiComments(
    tenantId,
    page,
    count,
    textSearch,
    byIPFromComment,
    filters,
    searchFilters,
    sorts,
    demo,
    sso
  );
}

fetchComments();
[inline-code-end]

---