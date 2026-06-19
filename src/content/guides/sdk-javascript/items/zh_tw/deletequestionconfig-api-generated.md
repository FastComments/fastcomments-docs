---
## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'deleteQuestionConfig 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f2b3c';
const id: string = 'qcfg_9a8b7c';
const metadataNote: string | undefined = undefined; // 選填的 metadata（此函式不需要）
const result: APIEmptyResponse = await deleteQuestionConfig(tenantId, id);
metadataNote;
[inline-code-end]

---