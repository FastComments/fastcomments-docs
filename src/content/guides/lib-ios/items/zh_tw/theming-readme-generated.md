### 主題預設

內建四種預設主題：

```swift
// 系統預設
sdk.theme = FastCommentsTheme.default

// 帶陰影與大圓角的卡片
sdk.theme = FastCommentsTheme.modern

// 平面、無陰影、小圓角、無討論串線
sdk.theme = FastCommentsTheme.minimal

// 將所有動作顏色設為單一品牌色
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### 留言顯示樣式

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // 帶分隔線的平面列表（預設）
theme.commentStyle = .card    // 帶陰影的圓角卡片
theme.commentStyle = .bubble  // 聊天泡泡樣式
```

### 顏色

所有顏色屬性皆為選填。未設定的值會回退到合理的系統預設值。

```swift
var theme = FastCommentsTheme()

// 品牌顏色
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// 背景
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// 動作按鈕
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

### 字體排版

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### 版面配置與間距

```swift
theme.cornerRadius = .large       // .none、.small、.medium、.large
theme.commentSpacing = 4          // 留言列之間的點數
theme.nestingIndent = 20          // 每個巢狀層級的縮排點數
theme.avatarSize = 36             // 根留言的頭像直徑
theme.replyAvatarSize = 28        // 巢狀回覆的頭像直徑
```

### 視覺效果

```swift
theme.showShadows = true          // 卡片上的細微陰影
theme.showThreadLine = true       // 連接巢狀回覆的垂直線
theme.animateVotes = true         // 投票變更時的彈簧動畫
```

### 套用主題

兩種方式：

```swift
// 透過 SwiftUI 環境（建議用於視圖階層）
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// 直接在 SDK 上設定
sdk.theme = theme
```

---


### 投票顏色

投票預設使用網頁小工具的投票卡片顏色，並支援淺色與深色模式。設定 `primaryColor` 也會同時變更強調色以及你所選選項的長條。若要直接設定：

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| 屬性 | 用途 |
|----------|----------|
| `pollAccentColor` | 選取選項的邊框與勾選標記、所選時長、「Your vote is in」 |
| `pollAccentTextColor` | 繪製在強調色上的勾選標記與文字 |
| `pollOptionBackgroundColor` | 選項標籤背景 |
| `pollOptionPressedBackgroundColor` | 按下時的選項標籤背景 |
| `pollBarColor` | 非你所選選項的結果長條 |
| `pollSelectedBarColor` | 你所選選項的結果長條 |
| `pollBorderColor` | 核取方塊標記與外框膠囊 |
| `pollDangerColor` | 錯誤與「Remove poll」 |