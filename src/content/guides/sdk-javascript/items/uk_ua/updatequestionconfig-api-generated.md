## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|------------|------|
| tenantId | string | Так |  |
| id | string | Так |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Так |  |

## Відповідь

Повертає: [`UpdateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateQuestionConfigResponse.ts)

## Приклад

[inline-code-attrs-start title = 'updateQuestionConfig Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const questionId: string = "qstn-2023-04";

const updateBody: UpdateQuestionConfigBody = {
  // продемонстровані необов'язкові поля
  customOptions: [
    {
      id: "opt-001",
      label: "Extra Details",
      required: true,
    },
  ],
  renderingType: "markdown",
};

const response: UpdateQuestionConfigResponse = await updateQuestionConfig(
  tenantId,
  questionId,
  updateBody
);
[inline-code-end]