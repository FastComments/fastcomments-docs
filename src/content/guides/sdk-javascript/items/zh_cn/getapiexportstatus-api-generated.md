## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| batchJobId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## 示例

[inline-code-attrs-start title = 'getApiExportStatus 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "tenant_12345";
  const batchJobId: string = "job_98765";
  const ssoToken: string = "sso_abcde12345";

  const statusOnlyTenant: ModerationExportStatusResponse = await getApiExportStatus(tenantId);
  const statusWithBatch: ModerationExportStatusResponse = await getApiExportStatus(tenantId, batchJobId);
  const statusFull: ModerationExportStatusResponse = await getApiExportStatus(tenantId, batchJobId, ssoToken);
}
run();
[inline-code-end]