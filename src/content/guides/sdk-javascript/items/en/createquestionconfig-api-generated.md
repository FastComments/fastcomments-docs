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
const tenantId: string = "acme-enterprises";
const createQuestionConfigBody: CreateQuestionConfigBody = {
  title: "Product Satisfaction",
  description: "Quarterly feedback on overall product experience",
  isActive: true,
  customOptions: [
    { label: "Very satisfied", value: "5" },
    { label: "Somewhat satisfied", value: "4" }
  ] // optional fields like description are shown; others omitted
};
const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]
