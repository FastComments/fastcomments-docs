The Recent Comments widget displays a list of the most recent comments across your site or for a specific page.

This widget includes minimal default styling and is designed to be easily customized with your own CSS.

## Widget Structure

The widget renders with the following HTML structure:

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

## Recent Comments Default CSS Reference

The widget includes the following minimal default styling:

[inline-code-attrs-start title = 'Recent Comments Widget Default CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
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

## Customization Examples

### Change Avatar Size
```css
.fastcomments-recent-comments .comment .user-details img {
    width: 32px !important;
    height: 32px !important;
    border-radius: 50%;
}
```

### Change Comment Text Truncation
The default styling uses CSS tricks to truncate long comments with "...". To disable:

```css
.fastcomments-recent-comments .comment .comment-text:before,
.fastcomments-recent-comments .comment .comment-text:after {
    display: none !important;
}
```

### Add Border to Comments
```css
.fastcomments-recent-comments .comment {
    border-bottom: 1px solid #eee !important;
    padding-bottom: 10px !important;
}
```