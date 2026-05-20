## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateQuestionConfig Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-media";
const id: string = "qc-20260520-01";

const updateQuestionConfigBody: UpdateQuestionConfigBody = {
  title: "Article Feedback",
  enabled: true,
  renderingType: "single" as QuestionRenderingType,
  whenSave: "immediate" as QuestionWhenSave,
  customOptions: [
    { id: "opt_yes", label: "Helpful" } as QuestionConfigCustomOptionsInner,
    { id: "opt_no", label: "Not Helpful" } as QuestionConfigCustomOptionsInner
  ]
};

const result: FlagCommentPublic200Response = await updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
[inline-code-end]
