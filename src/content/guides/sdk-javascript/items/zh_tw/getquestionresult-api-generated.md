## 參數

| 名稱 | 類型 | 是否必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 回應

回傳: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## 範例

[inline-code-attrs-start title = 'getQuestionResult 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const post: { title: string; questionId?: string } = { title: 'Product feedback' };
const tenantId: string = 'acme-corp-tenant-01';
const id: string = post.questionId ?? 'q-8f3a7b2c4d9e';
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, id);
[inline-code-end]

---