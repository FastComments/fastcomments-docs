显示带评论的社交媒体风格信息流：

```java
// 配置 SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// 初始化 Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// 设置信息流视图
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// 设置交互监听器
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // 信息流加载成功
    }

    @Override
    public void onFeedError(String errorMessage) {
        // 处理错误
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // 用户选择了帖子
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // 显示该帖子的评论
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// 加载信息流
feedView.load();
```