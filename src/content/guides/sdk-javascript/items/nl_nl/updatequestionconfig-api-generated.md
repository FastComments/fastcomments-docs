## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Ja |  |

## Reactie

Retourneert: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateQuestionConfig Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const questionId: string = "question-42";

  const updateBody: UpdateQuestionConfigBody = {
    title: "Revised FAQ Question"
    // isActive, customOptions, enz. zijn optioneel en weggelaten
  };

  const response: APIEmptyResponse = await updateQuestionConfig(tenantId, questionId, updateBody);
  console.log(response);
})();
[inline-code-end]