## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Ja |  |

## Antwort

Gibt zurück: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für updateQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f8b3c';
const id: string = 'questionResult_4621';
const updateQuestionResultBody: UpdateQuestionResultBody = {
  questionId: 'q_1024',
  result: 'flagged',
  score: 0.92,
  notes: 'Automated moderation flagged for review',
  meta: [{ key: 'source', value: 'ai-moderator' }] as MetaItem[], // optionale Metadaten
  status: { code: 'review_pending' } as APIStatus
} as UpdateQuestionResultBody;
const result: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---