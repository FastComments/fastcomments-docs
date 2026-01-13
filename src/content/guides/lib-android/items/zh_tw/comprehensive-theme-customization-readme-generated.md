FastComments SDK 中的所有按鈕與 UI 元素都可套用主題。使用 `FastCommentsTheme.Builder` 可完整控管您應用程式的品牌樣式。

### 程式化主題設定（建議）

```kotlin
val theme = FastCommentsTheme.Builder()
    // 操作按鈕：發送、投票、選單、按讚/分享按鈕
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // 回覆按鈕：留言回覆按鈕  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // 切換按鈕：顯示/隱藏回覆按鈕
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // 載入更多按鈕：分頁按鈕
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// 套用主題
sdk.setTheme(theme)
```

### 快速顏色覆寫

在你的 `colors.xml` 中覆寫顏色資源即可達成簡易品牌設定：

```xml
<!-- 在你的應用程式的 res/values/colors.xml 中 -->
<resources>
    <!-- 變更所有主要 UI 元素 -->
    <color name="primary">#FF1976D2</color>
    
    <!-- 或自訂特定按鈕類型 -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### 主題化按鈕涵蓋範圍

**SDK 中的每一個按鈕都支援主題化：**
- 發送按鈕、投票按鈕、選單按鈕、回覆按鈕
- 顯示/隱藏回覆按鈕、載入更多按鈕  
- 動態牆的操作按鈕（按讚、留言、分享）
- 對話框按鈕（提交、取消、儲存）
- 動態貼文中的動態任務按鈕

如需更詳細的主題文件，請參見 [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).