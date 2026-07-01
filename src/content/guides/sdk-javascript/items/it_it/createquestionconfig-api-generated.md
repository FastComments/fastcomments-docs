## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Sì |  |

## Risposta

Restituisce: [`CreateQuestionConfigResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfigResponse1.ts)

## Esempio

[inline-code-attrs-start title = 'createQuestionConfig Esempio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const customOption: QuestionConfigCustomOptionsInner = {
  label: "Option A",
  value: "a",
};

const createQuestionConfigBody: CreateQuestionConfigBody = {
  questionText: "What is your favorite color?",
  isActive: true,
  // i campi opzionali possono essere omessi
  customOptions: [customOption],
};

const response: CreateQuestionConfigResponse1 = await createQuestionConfig(
  tenantId,
  createQuestionConfigBody
);
[inline-code-end]