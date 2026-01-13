Prikaži feed u stilu društvenih mreža s komentarima:

```java
// Konfiguriraj SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Inicijaliziraj Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Postavi prikaz feeda
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Postavi slušatelja interakcija
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Feed je uspješno učitan
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Obradi pogreške
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Korisnik je odabrao objavu
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