## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| sso | string | No |  |

## 応答

返却: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportResponse.ts)

## 例

[inline-code-attrs-start title = 'postApiExport 例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---