---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| fromName | string | 是 |  |

## 回應

回傳: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'sendInvite 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'bright-media-12';
const id: string = 'user-8f4d2';
const fromName: string = 'Sofia Park';
const optionalNote: string | undefined = undefined;
const result: APIEmptyResponse = await sendInvite(tenantId, id, fromName);
[inline-code-end]

---