コメント付きのソーシャルメディア風フィードを表示する:

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
        // フィードの読み込みに成功
    }

    @Override
    public void onFeedError(String errorMessage) {
        // エラーを処理する
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // ユーザーが投稿を選択した
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // 投稿のコメントを表示する
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Load the feed
feedView.load();
```