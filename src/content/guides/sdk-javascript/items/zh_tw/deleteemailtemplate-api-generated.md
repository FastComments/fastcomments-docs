## 參數

| 名稱 | 類型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'deleteEmailTemplate 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9c4f1b2a";
const id: string = "emailtmpl_4d2b9a5e";
const requestorNote: string | undefined = undefined; // 選用的額外資料（函式不需要）
const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, id);
[inline-code-end]

---