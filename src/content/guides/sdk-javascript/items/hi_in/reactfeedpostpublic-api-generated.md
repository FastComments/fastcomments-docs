## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| postId | string | हाँ |  |
| reactBodyParams | ReactBodyParams | हाँ |  |
| isUndo | boolean | नहीं |  |
| broadcastId | string | नहीं |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'reactFeedPostPublic उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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