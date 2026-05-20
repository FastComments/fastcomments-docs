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
const tenantId: string = "acme-marketing";
const id: string = "7f3a2c1e-4b6d-4c9f-8a2e-0d9b6f1c2e3a";
const updateQuestionResultBody: UpdateQuestionResultBody = {
  verdict: "correct",
  note: "Peer-reviewed and confirmed",
  meta: [] as MetaItem[] // optional metadata omitted/empty
} as UpdateQuestionResultBody;
const result: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);

[inline-code-end]
