---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Да |  |

## Отговор

Връща: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfig200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f3b1c2a";

const createQuestionConfigBody: CreateQuestionConfigBody = {
  name: "Product feedback",
  key: "product_quality",
  description: "Short survey question shown after posting a comment",
  required: true,
  renderingType: "singleChoice",
  customOptions: [
    { label: "Excellent", value: "5" },
    { label: "Good", value: "4" },
    { label: "Fair", value: "3" }
  ] as QuestionConfigCustomOptionsInner[],
  notifyModerators: false // показан е опционален параметър
};

const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]

---