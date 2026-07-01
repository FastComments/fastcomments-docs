## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| textSearch | string | Hayır |  |
| byIPFromComment | string | Hayır |  |
| filters | string | Hayır |  |
| searchFilters | string | Hayır |  |
| sorts | string | Hayır |  |
| tenantId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`PostApiExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostApiExportResponse.ts)

## Örnek

[inline-code-attrs-start title = 'postApiExport Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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