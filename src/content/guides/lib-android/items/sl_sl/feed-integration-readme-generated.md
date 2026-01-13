---
Prikaži vir v slogu družbenih omrežij s komentarji:

```java
// Konfiguriraj SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Inicializiraj Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Nastavi pogled vira
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Nastavi poslušalca interakcij
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Vir je bil uspešno naložen
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Obravnavaj napake
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Uporabnik je izbral objavo
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Prikaži komentarje za objavo
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Naloži vir
feedView.load();
```
---