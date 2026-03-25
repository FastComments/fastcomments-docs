## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Ja |  |

## Antwort

Gibt zurück: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfig200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'createQuestionConfig Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b2c";
const option: QuestionConfigCustomOptionsInner = { id: "opt_yes", label: "Yes, helpful", value: "yes" };
const createQuestionConfigBody: CreateQuestionConfigBody = {
  title: "Article usefulness",
  prompt: "Was this article helpful?",
  type: "singleChoice",
  required: false, // optionaler Parameter demonstriert
  options: [option],
  saveBehavior: "immediate"
};
const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]