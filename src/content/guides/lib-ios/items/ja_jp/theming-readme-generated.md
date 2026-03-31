### テーマプリセット

4つの組み込みプリセットが利用可能です:

```swift
// システムのデフォルト
sdk.theme = FastCommentsTheme.default

// 影付きのカード、大きな角丸
sdk.theme = FastCommentsTheme.modern

// フラット、影なし、小さな角丸、スレッド線なし
sdk.theme = FastCommentsTheme.minimal

// すべてのアクションカラーを単一のブランドカラーに設定
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### コメント表示スタイル

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // 仕切り付きのフラットリスト（デフォルト）
theme.commentStyle = .card    // 影付きの角丸カード
theme.commentStyle = .bubble  // チャットバブルスタイル
```

### カラー

すべてのカラー属性は任意です。未設定の値は適切なシステムのデフォルトにフォールバックします。

```swift
var theme = FastCommentsTheme()

// ブランドカラー
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// 背景
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// アクションボタン
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// 投票
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// リンク
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// ダイアログ
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// 入力バー
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// その他
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### タイポグラフィ

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### レイアウトと間隔

```swift
theme.cornerRadius = .large       // 使用可能: .none, .small, .medium, .large
theme.commentSpacing = 4          // コメント行間のポイント数
theme.nestingIndent = 20          // ネストレベルごとのインデント量（ポイント）
theme.avatarSize = 36             // ルートコメントのアバター直径
theme.replyAvatarSize = 28        // ネストされた返信のアバター直径
```

### 視覚効果

```swift
theme.showShadows = true          // カードに薄い影を表示
theme.showThreadLine = true       // ネストされた返信をつなぐ垂直線
theme.animateVotes = true         // 投票変更時のスプリングアニメーション
```

### テーマの適用

2つの方法:

```swift
// SwiftUI の環境経由（ビュー階層には推奨）
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// SDK に直接設定
sdk.theme = theme
```