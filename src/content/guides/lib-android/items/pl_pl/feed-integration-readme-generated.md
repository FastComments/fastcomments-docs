Wyświetl kanał w stylu mediów społecznościowych z komentarzami:

```java
// Skonfiguruj SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Zainicjalizuj Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Skonfiguruj widok kanału
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Ustaw nasłuchiwacz interakcji
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Kanał załadowany pomyślnie
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Obsłuż błędy
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Użytkownik wybrał wpis
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Pokaż komentarze dla wpisu
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Załaduj kanał
feedView.load();
```