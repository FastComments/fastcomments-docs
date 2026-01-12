## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| contextUserId | string | No |  |
| isLive | boolean | No |  |

## Response

Returns: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteComment200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run(): Promise<void> {
  const result: DeleteComment200Response = await deleteComment(
    'acme-corp-tenant-01',
    'comment-9f2b3c',
    'user-4827',
    true
  );
  console.log(result);
}
run();
[inline-code-end]
