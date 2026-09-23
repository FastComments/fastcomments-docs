## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Oui |  |

## Réponse

Retourne : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de updateQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp-tenant";
  const questionId: string = "question-42";

  const updateBody: UpdateQuestionConfigBody = {
    title: "Revised FAQ Question"
    // isActive, customOptions, etc. sont optionnels et omis
  };

  const response: APIEmptyResponse = await updateQuestionConfig(tenantId, questionId, updateBody);
  console.log(response);
})();
[inline-code-end]