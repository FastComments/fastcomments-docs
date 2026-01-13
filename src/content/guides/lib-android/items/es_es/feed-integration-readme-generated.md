Muestra un feed al estilo de redes sociales con comentarios:

```java
// Configurar el SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Inicializar el Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Configurar la vista del feed
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Establecer el listener de interacción
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Feed cargado correctamente
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Manejar errores
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Usuario seleccionó una publicación
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Mostrar comentarios de la publicación
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Cargar el feed
feedView.load();
```