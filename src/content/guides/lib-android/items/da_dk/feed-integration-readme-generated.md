Vis en socialmedie-lignende feed med kommentarer:

```java
// Konfigurer SDK'en
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Initialiser Feed SDK'en
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Opsæt feed-visningen
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Sæt interaktionslytter
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Feed indlæst med succes
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Håndter fejl
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Brugeren valgte et indlæg
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Vis kommentarer for indlægget
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Load the feed
feedView.load();
```