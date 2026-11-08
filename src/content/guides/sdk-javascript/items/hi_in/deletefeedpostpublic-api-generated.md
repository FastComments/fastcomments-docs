## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| postId | string | हाँ |  |
| broadcastId | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublicResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'deleteFeedPostPublic उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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