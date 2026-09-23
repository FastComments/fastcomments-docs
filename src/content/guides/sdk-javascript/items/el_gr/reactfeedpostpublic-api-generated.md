## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| postId | string | Ναι |  |
| reactBodyParams | ReactBodyParams | Ναι |  |
| isUndo | boolean | Όχι |  |
| broadcastId | string | Όχι |  |
| sso | string | Όχι |  |

## Response

Επιστρέφει: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostResponse.ts)

## Example

[inline-code-attrs-start title = 'reactFeedPostPublic Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoReact() {
  const tenantId: string = "tenant_12345";
  const postId: string = "post_98765";
  const reactBodyParams: ReactBodyParams = {
    type: "like",
    userId: "user_abcde"
  };
  const response: ReactFeedPostResponse = await reactFeedPostPublic(
    tenantId,
    postId,
    reactBodyParams,
    true,
    "broadcast_001",
    "sso_token_xyz"
  );
}
[inline-code-end]

---