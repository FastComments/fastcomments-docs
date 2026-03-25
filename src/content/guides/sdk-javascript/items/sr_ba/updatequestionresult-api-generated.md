---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Yes |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primjer

[inline-code-attrs-start title = 'updateQuestionResult Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f8b3c';
const id: string = 'questionResult_4621';
const updateQuestionResultBody: UpdateQuestionResultBody = {
  questionId: 'q_1024',
  result: 'flagged',
  score: 0.92,
  notes: 'Automated moderation flagged for review',
  meta: [{ key: 'source', value: 'ai-moderator' }] as MetaItem[], // neobavezni metapodaci
  status: { code: 'review_pending' } as APIStatus
} as UpdateQuestionResultBody;
const result: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]

---