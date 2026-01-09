The Recent Comments 위젯은 사이트 전체 또는 특정 페이지에 대한 최신 댓글 목록을 표시합니다.

이 위젯은 최소한의 기본 스타일을 포함하며 사용자의 CSS로 쉽게 커스터마이즈할 수 있도록 설계되었습니다.

## 위젯 구조

이 위젯은 다음과 같은 HTML 구조로 렌더링됩니다:

```html
<div class="fastcomments-recent-comments">
    <div class="comment">
        <div class="user-details">
            <img src="..." alt="Avatar" class="avatar" />
            <span class="user-name">Username</span>
            <span class="reply-date-time">2 hours ago</span>
        </div>
        <div class="comment-text">Comment content...</div>
        <div class="link-wrapper">
            <a class="link" href="...">Page Title</a>
        </div>
    </div>
    <!-- More comments... -->
</div>
```

## Recent Comments 기본 CSS 참조

이 위젯은 다음과 같은 최소한의 기본 스타일을 포함합니다:

[inline-code-attrs-start title = 'Recent Comments 위젯 기본 CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
.fastcomments-recent-comments {
    font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,Cantarell,"Open Sans","Helvetica Neue",sans-serif;
    text-align: left;
}
.fastcomments-recent-comments .comment {
    padding-top: 5px;
}
.fastcomments-recent-comments .comment .user-details img {
    width: 20px;
    margin-right: 5px;
    vertical-align: middle;
}
.fastcomments-recent-comments .comment .user-details .user-name {
    vertical-align: middle;
    font-size: 12px;
}
.fastcomments-recent-comments .comment .user-details .reply-date-time {
    vertical-align: middle;
    padding-left: 5px;
    font-size: 10px;
}
.fastcomments-recent-comments .comment .comment-text {
    position: relative;
    line-height: 22px;
    font-size: 14px;
    text-align: justify;
    margin: 8px -1em 8px 0;
    padding-right: 1em;
}
.fastcomments-recent-comments .comment .comment-text .inline-image {
    display: block;
    max-width: 500px;
    margin: 3px 0 3px 0;
}
.fastcomments-recent-comments .comment .comment-text .inline-image img {
    max-width: 100%;
    max-height: 400px;
}
.fastcomments-recent-comments .comment .comment-text:before {
    position: absolute;
    content: "...";
    right: 0;
    bottom: 0;
}
.fastcomments-recent-comments .comment .comment-text:after {
    position: absolute;
    content: "";
    right: 0;
    width: 1em;
    height: 1em;
    margin-top: 0.2em;
    background: #fff;
}
.fastcomments-recent-comments .comment > .link-wrapper {
    margin: 5px 0 0 0;
}
.fastcomments-recent-comments .comment > .link-wrapper .link {
    font-size: 13px;
}
[inline-code-end]

## 사용자 정의 예제

### 아바타 크기 변경
```css
.fastcomments-recent-comments .comment .user-details img {
    width: 32px !important;
    height: 32px !important;
    border-radius: 50%;
}
```

### 댓글 텍스트 잘림 변경
기본 스타일은 CSS 트릭을 사용하여 긴 댓글을 "..."로 잘라냅니다. 비활성화하려면:

```css
.fastcomments-recent-comments .comment .comment-text:before,
.fastcomments-recent-comments .comment .comment-text:after {
    display: none !important;
}
```

### 댓글에 테두리 추가
```css
.fastcomments-recent-comments .comment {
    border-bottom: 1px solid #eee !important;
    padding-bottom: 10px !important;
}
```