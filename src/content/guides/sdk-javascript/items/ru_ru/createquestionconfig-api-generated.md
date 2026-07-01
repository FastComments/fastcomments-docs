## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Да |  |

## Ответ

Возвращает: [`CreateQuestionConfigResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfigResponse1.ts)

## Пример

[inline-code-attrs-start title = 'createQuestionConfig Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const customOption: QuestionConfigCustomOptionsInner = {
  label: "Option A",
  value: "a",
};

const createQuestionConfigBody: CreateQuestionConfigBody = {
  questionText: "What is your favorite color?",
  isActive: true,
  // optional fields can be omitted
  customOptions: [customOption],
};

const response: CreateQuestionConfigResponse1 = await createQuestionConfig(
  tenantId,
  createQuestionConfigBody
);
[inline-code-end]