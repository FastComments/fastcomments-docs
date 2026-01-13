Toon een feed in sociale-mediastijl met reacties:

```java
// Configureer de SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Initialiseer de Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Stel de feedweergave in
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Stel de listener voor interacties in
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Feed succesvol geladen
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Fouten afhandelen
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Gebruiker heeft een bericht geselecteerd
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Toon reacties voor het bericht
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Laad de feed
feedView.load();
```