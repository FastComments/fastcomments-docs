## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| createFeedPostParams | CreateFeedPostParams | Sim |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`CreateFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateFeedPostPublicResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo createFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function submitPost() {
  const tenantId: string = "tenant_12345";

  const mediaItem: FeedPostMediaItem = {
    asset: {
      url: "https://cdn.example.com/image.jpg",
      mimeType: "image/jpeg",
      size: 104857,
    },
    type: "image",
  };

  const link: FeedPostLink = {
    url: "https://example.com/blog/new-feature",
    title: "New Feature Release",
    description: "Details about our latest product update.",
  };

  const createFeedPostParams: CreateFeedPostParams = {
    content: "Check out our new feature!",
    media: [mediaItem],
    link: link,
  };

  const broadcastId: string = "broadcast_9876";
  const sso: string = "sso_token_abc123";

  const response: CreateFeedPostPublicResponse = await createFeedPostPublic(
    tenantId,
    createFeedPostParams,
    broadcastId,
    sso
  );
}
[inline-code-end]

---