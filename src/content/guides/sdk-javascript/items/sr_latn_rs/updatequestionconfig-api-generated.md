## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Da |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Da |  |

## Odgovor

Vraća: [`UpdateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateQuestionConfigResponse.ts)

## Primer

[inline-code-attrs-start title = 'updateQuestionConfig Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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