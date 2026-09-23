## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| textSearch | string | 아니오 |  |
| byIPFromComment | string | 아니오 |  |
| filters | string | 아니오 |  |
| searchFilters | string | 아니오 |  |
| sorts | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportResponse.ts)

## 예시

[inline-code-attrs-start title = 'postApiExport 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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