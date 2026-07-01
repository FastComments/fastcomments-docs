## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 回應

回傳: [`PostApiExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostApiExportResponse.ts)

## 範例

[inline-code-attrs-start title = 'postApiExport 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const textSearch: string = "keyword:feedback"
  const byIPFromComment: string = "203.0.113.45"
  const filters: string = "status:pending,category:support"
  const searchFilters: string = "createdAt>2023-01-01"
  const sorts: string = "createdAt:desc"
  const tenantId: string = "tenant_9876"
  const sso: string = "sso_7e2a9b"

  const exportResult: PostApiExportResponse = await postApiExport(
    textSearch,
    byIPFromComment,
    filters,
    searchFilters,
    sorts,
    tenantId,
    sso
  )

  console.log(exportResult)
})()
[inline-code-end]