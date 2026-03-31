### 主題預設

提供四個內建預設：

```swift
// 系統預設
sdk.theme = FastCommentsTheme.default

// 有陰影且大圓角的卡片
sdk.theme = FastCommentsTheme.modern

// 扁平，無陰影，小圓角，無討論串線
sdk.theme = FastCommentsTheme.minimal

// 將所有操作顏色設定為單一品牌色
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### 評論顯示樣式

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // 扁平列表，含分隔線（預設）
theme.commentStyle = .card    // 帶陰影的圓角卡片
theme.commentStyle = .bubble  // 聊天泡泡樣式
```

### 顏色

所有顏色屬性都是選填。未設定的值會回退到合適的系統預設。

```swift
var theme = FastCommentsTheme()

// 品牌色
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// 背景
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// 操作按鈕
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// 投票
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// 連結
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// 對話框
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// 輸入列
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

### 佈局與間距

```swift
theme.cornerRadius = .large       // 可選：.none, .small, .medium, .large
theme.commentSpacing = 4          // 評論列之間的間距（單位：點）
theme.nestingIndent = 20          // 每層巢狀的縮排（單位：點）
theme.avatarSize = 36             // 根評論的頭像直徑
theme.replyAvatarSize = 28        // 巢狀回覆的頭像直徑
```

### 視覺效果

```swift
theme.showShadows = true          // 卡片上的細微陰影
theme.showThreadLine = true       // 連接巢狀回覆的垂直線
theme.animateVotes = true         // 投票變化時的彈簧動畫
```

### 套用主題

兩種方式：

```swift
// 透過 SwiftUI 環境（建議用於檢視層級）
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// 直接在 SDK 上設定
sdk.theme = theme
```

---
---