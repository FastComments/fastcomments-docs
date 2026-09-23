## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| textSearch | string | 否 |  |
| byIPFromComment | string | 否 |  |
| filters | string | 否 |  |
| searchFilters | string | 否 |  |
| sorts | string | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`ModerationExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportResponse.ts)

## 範例

[inline-code-attrs-start title = 'postApiExport 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExport() {
  const tenantId: string = "c9f1e2b4-8a6d-4f3a-9d2e-5b6c7d8e9f0a";
  const textSearch: string = "spam";
  const byIPFromComment: string = "203.0.113.45";
  const filters: string = "status:pending";
  const searchFilters: string = "createdAt>2023-01-01";
  const sorts: string = "createdAt:desc";
  const sso: string = "sso-token-abc123";

  const fullExport: ModerationExportResponse = await postApiExport(
    tenantId,
    textSearch,
    byIPFromComment,
    filters,
    searchFilters,
    sorts,
    sso
  );

  const minimalExport: ModerationExportResponse = await postApiExport(tenantId);

  console.log(fullExport, minimalExport);
}
runExport();
[inline-code-end]