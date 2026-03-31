### 主题预设

提供四种内置预设：

```swift
// 系统默认
sdk.theme = FastCommentsTheme.default

// 带阴影和大圆角的卡片
sdk.theme = FastCommentsTheme.modern

// 扁平，无阴影，小圆角，无回复线
sdk.theme = FastCommentsTheme.minimal

// 将所有操作颜色设置为单一品牌颜色
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### 评论显示样式

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // 带分隔线的扁平列表（默认）
theme.commentStyle = .card    // 带阴影的圆角卡片
theme.commentStyle = .bubble  // 聊天气泡样式
```

### 颜色

所有颜色属性均为可选。未设置的值将回退到合理的系统默认值。

```swift
var theme = FastCommentsTheme()

// 品牌颜色
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// 背景
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// 操作按钮
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// 投票
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// 链接
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// 对话框
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// 输入栏
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// 其他
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### 排版

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### 布局与间距

```swift
theme.cornerRadius = .large       // 可选：.none、.small、.medium、.large
theme.commentSpacing = 4          // 评论行之间的间距（以点为单位）
theme.nestingIndent = 20          // 每个嵌套层级的缩进（以点为单位）
theme.avatarSize = 36             // 根评论的头像直径
theme.replyAvatarSize = 28        // 嵌套回复的头像直径
```

### 视觉效果

```swift
theme.showShadows = true          // 卡片上的细微阴影
theme.showThreadLine = true       // 连接嵌套回复的垂直线
theme.animateVotes = true         // 投票更改时的弹簧动画
```

### 应用主题

两种方法：

```swift
// 通过 SwiftUI 环境（推荐用于视图层次）
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// 直接在 SDK 上设置
sdk.theme = theme
```

---
---