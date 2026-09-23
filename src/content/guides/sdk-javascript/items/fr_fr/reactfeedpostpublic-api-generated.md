## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| postId | string | Oui |  |
| reactBodyParams | ReactBodyParams | Oui |  |
| isUndo | boolean | Non |  |
| broadcastId | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ReactFeedPostResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple reactFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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