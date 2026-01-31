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
const tenantId: string = "acme-publishing-01";
const id: string = "question_42b7";
const customOptions: QuestionConfigCustomOptionsInner[] = [
  { id: "opt_yes", label: "Yes", value: "yes" },
  { id: "opt_no", label: "No", value: "no" }
];
const updateQuestionConfigBody: UpdateQuestionConfigBody = {
  enabled: true,
  prompt: "Did this article answer your question?",
  required: false,
  options: customOptions,
  hint: "Select one option"
};
const result: FlagCommentPublic200Response = await updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
[inline-code-end]
