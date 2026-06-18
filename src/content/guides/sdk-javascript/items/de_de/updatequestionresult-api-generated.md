## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Ja |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für updateQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_84b2";
const id: string = "question_9f3a";
const updateQuestionResultBody: UpdateQuestionResultBody = {
  outcome: "accepted",
  confidence: 0.88,
  moderatorId: "moderator_17",
  notes: "Validated by automated review" // optionales Feld enthalten
};
const result: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---