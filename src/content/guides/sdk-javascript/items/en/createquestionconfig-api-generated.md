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
const tenantId: string = "acme-enterprises-789";

const createQuestionConfigBody: CreateQuestionConfigBody = {
  title: "Post-purchase feedback",
  enabled: true,
  customOptions: [{ key: "include_product_sku", label: "Include SKU" } as QuestionConfigCustomOptionsInner],
  tosConfig: undefined // optional parameter demonstrated
} as CreateQuestionConfigBody;

const result: CreateQuestionConfig200Response = await createQuestionConfig(tenantId, createQuestionConfigBody);
[inline-code-end]
