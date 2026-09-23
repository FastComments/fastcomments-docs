## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postId | string | Yes |  |
| reactBodyParams | ReactBodyParams | Yes |  |
| isUndo | boolean | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Yanıt

Döndürür: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostResponse.ts)

## Örnek

[inline-code-attrs-start title = 'reactFeedPostPublic Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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