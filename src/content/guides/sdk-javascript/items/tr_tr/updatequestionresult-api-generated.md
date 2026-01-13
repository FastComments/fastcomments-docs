## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Evet |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Örnek

[inline-code-attrs-start title = 'updateQuestionResult Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f2b9';
const id: string = 'question_3a7c1d';
const updateQuestionResultBody: UpdateQuestionResultBody = {
  result: { verdict: 'helpful', confidence: 0.92 },
  reviewer: { id: 'mod_102', name: 'Aisha Rahman' },
  notifyUser: true // isteğe bağlı parametre dahil edildi
};
const response: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---