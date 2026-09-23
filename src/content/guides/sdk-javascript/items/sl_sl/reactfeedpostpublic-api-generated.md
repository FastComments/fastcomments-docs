## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| postId | string | Da |  |
| reactBodyParams | ReactBodyParams | Da |  |
| isUndo | boolean | Ne |  |
| broadcastId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostResponse.ts)

## Primer

[inline-code-attrs-start title = 'reactFeedPostPublic Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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