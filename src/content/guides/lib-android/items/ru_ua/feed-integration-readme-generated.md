Отобразить ленту в стиле социальной сети с комментариями:

```java
// Настроить SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Инициализировать Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Настроить вид ленты
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Установить слушатель взаимодействия
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Лента успешно загружена
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Обработать ошибки
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Пользователь выбрал запись
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Показать комментарии для записи
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Загрузить ленту
feedView.load();
```