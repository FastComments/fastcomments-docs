Prikaži feed u stilu društvenih mreža sa komentarima:

```java
// Konfiguriši SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Inicijalizuj Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Podesi prikaz feeda
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Postavi listener za interakcije
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Feed je uspešno učitan
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Obradi greške
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Korisnik je izabrao objavu
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Prikaži komentare za objavu
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Učitaj feed
feedView.load();
```