## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f2b9';
const id: string = 'question_3a7c1d';
const updateQuestionResultBody: UpdateQuestionResultBody = {
  result: { verdict: 'helpful', confidence: 0.92 },
  reviewer: { id: 'mod_102', name: 'Aisha Rahman' },
  notifyUser: true // parametro opzionale incluso
};
const response: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---