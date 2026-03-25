## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Ja |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'updateQuestionConfig Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42e8b';
const id: string = 'question_9f4a2';
const updateQuestionConfigBody: UpdateQuestionConfigBody = {
  questionText: 'How helpful was this article?',
  description: 'Shown to users below the question (optional)',
  required: true,
  renderingType: 'Likert' as QuestionRenderingType,
  customOptions: [
    { label: 'Very helpful', value: '5' } as QuestionConfigCustomOptionsInner,
    { label: 'Somewhat helpful', value: '3' } as QuestionConfigCustomOptionsInner,
    { label: 'Not helpful', value: '1' } as QuestionConfigCustomOptionsInner
  ],
  whenSave: 'notify' as QuestionWhenSave
};
const result: FlagCommentPublic200Response = await updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
[inline-code-end]

---