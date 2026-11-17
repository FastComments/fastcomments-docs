### テーマプリセット

4つの組み込みプリセットが用意されています:

```swift
// システムのデフォルト
sdk.theme = FastCommentsTheme.default

// 影と大きな角丸を持つカード
sdk.theme = FastCommentsTheme.modern

// フラット、影なし、小さな角丸、スレッド線なし
sdk.theme = FastCommentsTheme.minimal

// すべてのアクションカラーを単一のブランドカラーに設定
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### コメント表示スタイル

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // 区切り線付きのフラットなリスト(デフォルト)
theme.commentStyle = .card    // 影付きの角丸カード
theme.commentStyle = .bubble  // チャットバブルスタイル
```

### カラー

すべてのカラープロパティはオプションです。未設定の値は適切なシステムデフォルトにフォールバックします。

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

### レイアウトとスペーシング

```swift
theme.cornerRadius = .large       // .none、.small、.medium、.large
theme.commentSpacing = 4          // コメント行間のポイント数
theme.nestingIndent = 20          // ネストレベルごとのインデントのポイント数
theme.avatarSize = 36             // ルートコメントのアバター直径
theme.replyAvatarSize = 28        // ネストされた返信のアバター直径
```

### 視覚効果

```swift
theme.showShadows = true          // カードのさりげない影
theme.showThreadLine = true       // ネストされた返信を結ぶ縦線
theme.animateVotes = true         // 投票変更時のスプリングアニメーション
```

### テーマの適用

2つのアプローチがあります:

```swift
// SwiftUI環境経由(ビュー階層に推奨)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// SDKに直接設定
sdk.theme = theme
```

---


### 投票のカラー

投票は、ライトモードとダークモードの両方で、Webウィジェットの投票カードのカラーをデフォルトとします。`primaryColor` を設定すると、アクセントと選択したオプションのバーも変更されます。直接設定するには:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| プロパティ | 用途 |
|----------|----------|
| `pollAccentColor` | 選択したオプションの枠線とチェック、選択した期間、「投票が記録されました」 |
| `pollAccentTextColor` | アクセントカラー上に描かれるチェックマークとテキスト |
| `pollOptionBackgroundColor` | オプションチップの背景 |
| `pollOptionPressedBackgroundColor` | 押下中のオプションチップの背景 |
| `pollBarColor` | 選択しなかったオプションの結果バー |
| `pollSelectedBarColor` | 選択したオプションの結果バー |
| `pollBorderColor` | チェックボックスのマークと枠線付きのピル |
| `pollDangerColor` | エラーと「投票を削除」 |