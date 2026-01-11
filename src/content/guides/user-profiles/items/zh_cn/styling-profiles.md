当用户在您网站的上下文中打开用户资料（通过评论小部件）时，您为 FastComments 小部件应用的任何自定义 CSS 样式都会自动注入到资料模态框中。

### 工作原理

当用户在您的评论小部件中点击资料链接时，会打开一个带有类 `.fast-comments-profile` 的资料模态框。您小部件的自定义 CSS 会自动注入到资料视图中。如果您已经为评论小部件添加了样式，这些样式也会应用到资料中。

### CSS 类

FastComments 的资料使用基于类的 CSS 架构。它不使用 CSS 自定义属性（custom properties）。

主资料页面使用 `.user-profile` 作为根容器。头部区域是 `.profile-header`，用于背景图片的是 `.profile-header-background`。资料内容位于 `.profile-content`。

头像使用 `.profile-avatar` 和 `.profile-avatar-wrapper`。用户的名称是 `.profile-name`，简介文本是 `.profile-bio`。统计信息位于 `.profile-stats`，单个统计项使用 `.stat`。

社交链接位于 `.profile-social-links`，单个链接为 `.social-link`。徽章使用 `.profile-badges` 和 `.badge`。徽章进度条使用 `.progress-outer` 和 `.progress-bar`。

选项卡容器使用 `.profile-tabs`，单个选项卡为 `.tab`，选中的选项卡为 `.tab.active`。选项卡内容使用 `.tab-body` 和 `.tab-body.active`。选项卡上的通知计数使用 `.tab .count`。

通知使用 `.notification`，私信对话使用 `.conversation`。在线状态为 `.activity-indicator`，活动状态为 `.activity-indicator.online`。未读计数使用 `.unread-count`。

资料模态框容器是 `.fast-comments-profile`，关闭按钮为 `.fast-comments-profile-close`。

### 深色模式

深色模式在 `.user-profile` 上使用 `.dark` 类修饰符。

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### 示例

**页眉:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**徽章:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**选项卡:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**模态框:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```