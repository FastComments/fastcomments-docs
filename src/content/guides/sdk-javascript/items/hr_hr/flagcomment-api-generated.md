## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| userId | string | Ne |  |
| anonUserId | string | Ne |  |

## Response

Vraća: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## Example

[inline-code-attrs-start title = 'flagComment Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-20230915-001";
  const userId: string = "user-42";
  const anonUserId: string = "anon-abc123";

  const response: FlagCommentResponse = await flagComment(tenantId, commentId, userId, anonUserId);
  console.log(response);
})();
[inline-code-end]

---