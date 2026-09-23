## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## レスポンス

返却: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublicResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteFeedPostPublic の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const postId: string = "post_98765";
  const broadcastId: string = "broadcast_abcde";
  const sso: string = "sso_token_xyz";

  const responseWithOpts: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(
    tenantId,
    postId,
    broadcastId,
    sso
  );

  const responseWithoutOpts: DeleteFeedPostPublicResponse = await deleteFeedPostPublic(
    tenantId,
    postId
  );
})();
[inline-code-end]