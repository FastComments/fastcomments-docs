## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Yes |  |

## Odgovor

Vraća: [`CreateQuestionConfigResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfigResponse1.ts)

## Primer

[inline-code-attrs-start title = 'createQuestionConfig Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const customOption: QuestionConfigCustomOptionsInner = {
  label: "Option A",
  value: "a",
};

const createQuestionConfigBody: CreateQuestionConfigBody = {
  questionText: "What is your favorite color?",
  isActive: true,
  // opcionalna polja se mogu izostaviti
  customOptions: [customOption],
};

const response: CreateQuestionConfigResponse1 = await createQuestionConfig(
  tenantId,
  createQuestionConfigBody
);
[inline-code-end]

---