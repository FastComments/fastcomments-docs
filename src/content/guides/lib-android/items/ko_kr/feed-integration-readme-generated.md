댓글이 포함된 소셜 미디어 스타일 피드 표시:

```java
// SDK 구성
CommentWidgetConfig config = new CommentWidgetConfig();
config.tenantId = "your-tenant-id";
config.urlId = "page-url-id";

// 피드 SDK 초기화
FastCommentsFeedSDK feedSDK = new FastCommentsFeedSDK(config);

// 피드 뷰 설정
FastCommentsFeedView feedView = findViewById(R.id.feedView);
feedView.setSDK(feedSDK);

// 상호작용 리스너 설정
feedView.setFeedViewInteractionListener(new FastCommentsFeedView.OnFeedViewInteractionListener() {
    @Override
    public void onFeedLoaded(List<FeedPost> posts) {
        // 피드 로드 성공
    }

    @Override
    public void onFeedError(String errorMessage) {
        // 오류 처리
    }

    @Override
    public void onPostSelected(FeedPost post) {
        // 사용자가 게시물을 선택함
    }

    @Override
    public void onCommentsRequested(FeedPost post) {
        // 게시물의 댓글 표시
        CommentsDialog dialog = new CommentsDialog(context, post, feedSDK);
        dialog.show();
    }
});

// Load the feed
feedView.load();
```