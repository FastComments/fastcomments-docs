टिप्पणियों के साथ सोशल मीडिया शैली का फ़ीड प्रदर्शित करें:

```java
// SDK कॉन्फ़िगर करें
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// Feed SDK प्रारंभ करें
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// फ़ीड व्यू सेट अप करें
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// इंटरैक्शन लिसनर सेट करें
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // फ़ीड सफलतापूर्वक लोड हुआ
    }

    @Override
    public void onFeedError(String errorMessage) {
        // त्रुटियों को संभालें
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // उपयोगकर्ता ने एक पोस्ट चुनी
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // पोस्ट के लिए टिप्पणियाँ दिखाएँ
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// फ़ीड लोड करें
feedView.load();
```