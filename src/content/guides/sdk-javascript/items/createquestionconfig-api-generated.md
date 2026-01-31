## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createQuestionConfigBody | CreateQuestionConfigBody | Yes |  |

## Response

Returns: [`CreateQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateQuestionConfig200Response.ts)

## Example

[inline-code-attrs-start title = 'createQuestionConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7c2b4f";

const createQuestionConfigBody: CreateQuestionConfigBody = {
  key: "product_feedback",
  label: "Product feedback",
  required: false, // optional parameter demonstrated
  renderingType: QuestionRenderingType.ShortAnswer,
  order: 3,
  customOptions: [
    {
      id: "opt_feature",
      label: "Feature request",
      value: "feature_request"
    }
  ]
};

const response: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]
