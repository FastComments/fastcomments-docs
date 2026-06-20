## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| batchJobId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## 示例

[inline-code-attrs-start title = 'getApiExportStatus 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]

---