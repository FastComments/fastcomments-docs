## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateQuestionResultBody | UpdateQuestionResultBody | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'updateQuestionResult の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme-corp_01";
const id: string = "question_9f2d1b";
const metaItem: MetaItem = { key: "platform", value: "web" };
const status: APIStatus = { code: 0, message: "scored" };
const updateQuestionResultBody: UpdateQuestionResultBody = {
  score: 92,
  passed: true,
  status,
  meta: [metaItem] // オプションのフィールドの例示
};
const result: APIEmptyResponse = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---