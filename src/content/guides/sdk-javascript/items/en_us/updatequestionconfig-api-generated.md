## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Yes |  |

## Response

Returns: [`UpdateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateQuestionConfigResponse.ts)

## Example

[inline-code-attrs-start title = 'updateQuestionConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
