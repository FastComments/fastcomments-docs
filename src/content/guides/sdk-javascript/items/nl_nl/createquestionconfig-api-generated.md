## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Ja |  |

## Respons

Retourneert: [`CreateQuestionConfigResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfigResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createQuestionConfig Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const customOption: QuestionConfigCustomOptionsInner = {
  label: "Option A",
  value: "a",
};

const createQuestionConfigBody: CreateQuestionConfigBody = {
  questionText: "What is your favorite color?",
  isActive: true,
  // optionele velden kunnen weggelaten worden
  customOptions: [customOption],
};

const response: CreateQuestionConfigResponse1 = await createQuestionConfig(
  tenantId,
  createQuestionConfigBody
);
[inline-code-end]