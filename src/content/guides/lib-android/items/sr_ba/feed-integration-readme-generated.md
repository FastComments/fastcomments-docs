Прикажите фид у стилу друштвених мрежа са коментарима:

```java
// Конфигуришите SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Иницијализујте Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Подесите приказ фида
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Поставите слушача интеракције
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Фид је успешно учитан
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Обрадите грешке
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Корисник је одабрао објаву
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Прикажите коментаре за објаву
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Учитајте фид
feedView.load();
```