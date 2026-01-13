Visualizza un feed in stile social media con commenti:

```java
// Configura lo SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Inizializza il Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Configura la vista del feed
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Imposta il listener di interazione
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Feed caricato correttamente
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Gestisci gli errori
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // L'utente ha selezionato un post
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Mostra i commenti per il post
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Carica il feed
feedView.load();
```