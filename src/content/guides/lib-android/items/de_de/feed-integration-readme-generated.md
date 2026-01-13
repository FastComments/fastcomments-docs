Einen Social-Media-ähnlichen Feed mit Kommentaren anzeigen:

```java
// SDK konfigurieren
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Feed-SDK initialisieren
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Feed-Ansicht einrichten
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Interaktions-Listener festlegen
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Feed erfolgreich geladen
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Fehler behandeln
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Benutzer hat einen Beitrag ausgewählt
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Kommentare für den Beitrag anzeigen
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Feed laden
feedView.load();
```