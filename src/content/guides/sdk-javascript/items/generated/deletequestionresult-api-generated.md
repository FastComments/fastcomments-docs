## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteQuestionResult Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "org-82479";
const questionId: string = "q-6a3d2e1f";
const includeArchived?: boolean = false; // optional parameter demonstrating alternate target selection
const targetId: string = includeArchived ? "q-archived-112233" : questionId;
const result: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, targetId);
[inline-code-end]
