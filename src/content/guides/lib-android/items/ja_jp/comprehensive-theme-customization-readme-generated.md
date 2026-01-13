FastComments SDK のすべてのボタンと UI 要素はテーマ可能です。アプリのブランディングを完全に制御するには `FastCommentsTheme.Builder` を使用してください。

### プログラムによるテーマ設定（推奨）

```kotlin
val theme = FastCommentsTheme.Builder()
    // アクションボタン: 送信、投票、メニュー、いいね/共有ボタン
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // 返信ボタン: コメント返信ボタン  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // トグルボタン: 返信の表示/非表示ボタン
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // もっと読み込むボタン: ページネーションボタン
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// テーマを適用
sdk.setTheme(theme)
```

### 簡単なカラーの上書き

Override color resources in your `colors.xml` for simple branding:

```xml
<!-- アプリの res/values/colors.xml 内 -->
<resources>
    <!-- すべての主要な UI 要素を変更 -->
    <color name="primary">#FF1976D2</color>
    
    <!-- または特定のボタン種別をカスタマイズ -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### テーマ対応ボタンの範囲

**SDK のすべてのボタンでテーマがサポートされています:**
- 送信ボタン、投票ボタン、メニューボタン、返信ボタン
- 返信の表示/非表示ボタン、もっと読み込むボタン  
- フィードのアクションボタン（いいね、コメント、共有）
- ダイアログボタン（送信、キャンセル、保存）
- フィード投稿内の動的タスクボタン

詳細なテーマ設定のドキュメントは [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md) を参照してください。