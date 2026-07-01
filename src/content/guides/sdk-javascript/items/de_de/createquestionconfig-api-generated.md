## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Ja |  |

## Antwort

Rückgabe: [`CreateQuestionConfigResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfigResponse1.ts)

## Beispiel

[inline-code-attrs-start title = 'createQuestionConfig Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const customOption: QuestionConfigCustomOptionsInner = {
  label: "Option A",
  value: "a",
};

const createQuestionConfigBody: CreateQuestionConfigBody = {
  questionText: "What is your favorite color?",
  isActive: true,
  // optionale Felder können weggelassen werden
  customOptions: [customOption],
};

const response: CreateQuestionConfigResponse1 = await createQuestionConfig(
  tenantId,
  createQuestionConfigBody
);
[inline-code-end]