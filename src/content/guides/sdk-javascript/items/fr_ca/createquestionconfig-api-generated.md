## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Oui |  |

## Réponse

Renvoie: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfig200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de createQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
  notifyModerators: false // paramètre optionnel démontré
};

const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]