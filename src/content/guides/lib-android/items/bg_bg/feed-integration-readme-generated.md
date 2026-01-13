Показване на емисия в стил социални медии с коментари:

```java
// Конфигурирайте SDK-то
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Инициализирайте Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Настройте изгледа на емисията
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Задайте слушател за взаимодействия
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Емисията е заредена успешно
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Обработете грешките
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Потребителят избра публикация
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Покажете коментарите за публикацията
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Заредете емисията
feedView.load();
```