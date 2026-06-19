## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Evet |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateQuestionResult Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme-corp_01";
const id: string = "question_9f2d1b";
const metaItem: MetaItem = { key: "platform", value: "web" };
const status: APIStatus = { code: 0, message: "scored" };
const updateQuestionResultBody: UpdateQuestionResultBody = {
  score: 92,
  passed: true,
  status,
  meta: [metaItem] // isteğe bağlı alan gösterimi
};
const result: APIEmptyResponse = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---