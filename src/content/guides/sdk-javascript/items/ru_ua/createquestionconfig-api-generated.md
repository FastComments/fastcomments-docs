## Параметры

| Name | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Да |  |

## Ответ

Возвращает: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfig200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример createQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_01";
const createQuestionConfigBody: CreateQuestionConfigBody = {
  title: "Post-purchase feedback",
  description: "Quick survey about your recent order",
  required: true,
  renderingType: "single_choice",
  options: [
    { label: "Very dissatisfied", value: "1" },
    { label: "Dissatisfied", value: "2" },
    { label: "Neutral", value: "3" },
    { label: "Satisfied", value: "4" },
    { label: "Very satisfied", value: "5" }
  ] as QuestionConfigCustomOptionsInner[]
} as CreateQuestionConfigBody;
const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]

---