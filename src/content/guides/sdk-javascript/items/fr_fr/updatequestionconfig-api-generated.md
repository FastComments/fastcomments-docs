## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Oui |  |

## Réponse

Retourne : [`UpdateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateQuestionConfigResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple updateQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const questionId: string = "qstn-2023-04";

const updateBody: UpdateQuestionConfigBody = {
  // optional fields demonstrated
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

---