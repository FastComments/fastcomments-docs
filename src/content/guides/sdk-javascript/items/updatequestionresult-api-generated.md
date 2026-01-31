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
const tenantId: string = 'acme-corp-tenant-42';
const id: string = 'qres-20260109-7788';
const meta: MetaItem[] = [{ key: 'channel', value: 'web' }];
const updateQuestionResultBody: UpdateQuestionResultBody = {
  questionId: 'q-9876',
  score: 4,
  updatedBy: 'moderator.alex@acme.com',
  updatedAt: new Date().toISOString(),
  meta // optional metadata demonstrated
};
const result: FlagCommentPublic200Response = await updateQuestionResult(tenantId, id, updateQuestionResultBody);
[inline-code-end]
