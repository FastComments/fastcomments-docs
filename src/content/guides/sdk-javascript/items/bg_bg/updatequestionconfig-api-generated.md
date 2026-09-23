## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Да |  |

## Отговор

Връща: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Пример

[inline-code-attrs-start title = 'updateQuestionConfig Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const questionId: string = "question-42";

  const updateBody: UpdateQuestionConfigBody = {
    title: "Revised FAQ Question"
    // isActive, customOptions, и др. са незадължителни и са пропуснати
  };

  const response: APIEmptyResponse = await updateQuestionConfig(tenantId, questionId, updateBody);
  console.log(response);
})();
[inline-code-end]