---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Ja |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på updateQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_prod_84b2";
const id: string = "question_9f3a";
const updateQuestionResultBody: UpdateQuestionResultBody = {
  outcome: "accepted",
  confidence: 0.88,
  moderatorId: "moderator_17",
  notes: "Validated by automated review" // valgfrit felt inkluderet
};
const result: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---