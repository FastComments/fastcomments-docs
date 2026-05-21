## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## 例

[inline-code-attrs-start title = 'getQuestionResult の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const id: string = 'question-9f8b7c';
const includeComments: boolean | undefined = true; // オプションパラメータの例
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, id);
console.log(result);
[inline-code-end]

---