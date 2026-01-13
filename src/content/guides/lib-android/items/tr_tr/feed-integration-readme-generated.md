Yorumlarla birlikte sosyal medya tarzı bir akışı görüntüleyin:

```java
// SDK'yı yapılandır
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Feed SDK'sını başlat
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Akış görünümünü ayarla
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Etkileşim dinleyicisini ayarla
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Akış başarıyla yüklendi
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Hataları işle
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Kullanıcı bir gönderi seçti
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Gönderi için yorumları göster
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Akışı yükle
feedView.load();
```