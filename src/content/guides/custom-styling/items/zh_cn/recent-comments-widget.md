最近评论 小部件显示站点范围内或特定页面上最新评论的列表。

该小部件包含最小的默认样式，旨在便于使用您自己的 CSS 进行定制。

## 小部件结构

该小部件使用以下 HTML 结构渲染：

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

## 最近评论 默认 CSS 参考

该小部件包含以下最小默认样式：

[inline-code-attrs-start title = '最近评论 小部件 默认 CSS'; type = 'css'; isFunctional = false; inline-code-attrs-end]
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

## 自定义示例

### 更改头像大小
```css
.fastcomments-recent-comments .comment .user-details img {
    width: 32px !important;
    height: 32px !important;
    border-radius: 50%;
}
```

### 更改评论文本截断
默认样式使用 CSS 技巧通过 "..." 来截断较长的评论。要禁用：

```css
.fastcomments-recent-comments .comment .comment-text:before,
.fastcomments-recent-comments .comment .comment-text:after {
    display: none !important;
}
```

### 为评论添加边框
```css
.fastcomments-recent-comments .comment {
    border-bottom: 1px solid #eee !important;
    padding-bottom: 10px !important;
}
```