## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

返却: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## 例

[inline-code-attrs-start title = 'getQuestionResult の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const post: { title: string; questionId?: string } = { title: 'Product feedback' };
const tenantId: string = 'acme-corp-tenant-01';
const id: string = post.questionId ?? 'q-8f3a7b2c4d9e';
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, id);
[inline-code-end]

---