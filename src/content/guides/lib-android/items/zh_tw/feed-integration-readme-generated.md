顯示帶有評論的社群媒體風格動態：

```java
// Configure the SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Initialize the Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Set up the feed view
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Set interaction listener
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Feed loaded successfully
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Handle errors
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // User selected a post
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Show comments for the post
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Load the feed
feedView.load();
```