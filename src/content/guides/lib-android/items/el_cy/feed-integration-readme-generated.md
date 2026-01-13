Εμφάνιση ροής σε στυλ κοινωνικών μέσων με σχόλια:

```java
// Διαμόρφωση του SDK
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Αρχικοποίηση του Feed SDK
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// Ρύθμιση της προβολής ροής
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// Ορισμός listener αλληλεπίδρασης
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // Η ροή φορτώθηκε επιτυχώς
    }

    @Override
    public void onFeedError(String errorMessage) {
        // Χειρισμός σφαλμάτων
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // Ο χρήστης επέλεξε μία ανάρτηση
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // Εμφάνιση σχολίων για την ανάρτηση
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Φόρτωση της ροής
feedView.load();
```