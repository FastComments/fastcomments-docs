---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

返却値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteQuestionResult の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-01";
const id: string = "qres_9f8b7c3a";
const response: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, id);
const optionalResponse: FlagCommentPublic200Response | undefined = response;
[inline-code-end]

---