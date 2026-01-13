Afficher un fil de style réseau social avec des commentaires:

```java
// Configurer le SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Initialiser le Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Configurer la vue du fil
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Définir le gestionnaire d'interactions
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Fil chargé avec succès
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Gérer les erreurs
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // L'utilisateur a sélectionné une publication
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Afficher les commentaires pour la publication
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Charger le fil
feedView.load();
```