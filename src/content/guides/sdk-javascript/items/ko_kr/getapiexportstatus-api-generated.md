## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| batchJobId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## 예제

[inline-code-attrs-start title = 'getApiExportStatus 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]

---