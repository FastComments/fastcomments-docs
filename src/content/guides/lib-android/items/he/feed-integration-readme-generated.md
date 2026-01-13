הצג פיד בסגנון רשת חברתית עם תגובות:

```java
// הגדר את ה-SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// אתחל את Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// הגדר את תצוגת הפיד
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// הגדר מאזין לאינטראקציות
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // הפיד הוטען בהצלחה
    }

    @Override
    public void onFeedError(String errorMessage) {
        // טפל בשגיאות
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // המשתמש בחר פוסט
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // הצג תגובות עבור הפוסט
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// טען את הפיד
feedView.load();
```