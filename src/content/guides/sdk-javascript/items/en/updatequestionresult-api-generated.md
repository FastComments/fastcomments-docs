## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateQuestionResult Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const id: string = "qresult_98765";
const meta: MetaItem[] = [{ key: "source", value: "knowledge_base" }];
const updateQuestionResultBody: UpdateQuestionResultBody = {
  reviewerId: "user_2468",
  passed: true,
  score: 0.92,
  comment: "Reviewed and confirmed accuracy",
  meta,                // optional metadata included
  notifyFollowers: false // optional flag demonstrated
};
const result: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]
