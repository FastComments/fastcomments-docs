Exibir um feed no estilo de rede social com comentários:

```java
// Configurar o SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Inicializar o Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Configurar a visualização do feed
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Definir o listener de interação
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Feed carregado com sucesso
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Lidar com erros
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Usuário selecionou um post
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Mostrar comentários para o post
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Carregar o feed
feedView.load();
```