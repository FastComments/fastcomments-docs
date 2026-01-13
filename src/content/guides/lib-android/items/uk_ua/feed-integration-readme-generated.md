Відобразити стрічку у стилі соціальної мережі з коментарями:

```java
// Налаштувати SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Ініціалізувати Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Налаштувати відображення стрічки
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Встановити слухача взаємодії
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Стрічка успішно завантажена
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Обробити помилки
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Користувач вибрав допис
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Показати коментарі для допису
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Load the feed
feedView.load();
```