所有 FastComments SDK 中的按钮和 UI 元素都支持主题化。使用 `FastCommentsTheme.Builder` 完全控制您应用的品牌样式。

### 编程式主题（推荐）

```kotlin
val theme = FastCommentsTheme.Builder()
    // 操作按钮：发送、投票、菜单、点赞/分享 按钮
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // 回复按钮：评论回复按钮  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // 切换按钮：显示/隐藏回复 按钮
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // 加载更多按钮：分页按钮
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// 应用主题
sdk.setTheme(theme)
```

### 快速颜色覆盖

在您的 `colors.xml` 中覆盖颜色资源以实现简单的品牌定制：

```xml
<!-- 在您的应用的 res/values/colors.xml 中 -->
<resources>
    <!-- 更改所有主要 UI 元素 -->
    <color name="primary">#FF1976D2</color>
    
    <!-- 或自定义特定的按钮类型 -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### 主题按钮覆盖范围

**SDK 中的每个按钮都支持主题化：**
- 发送按钮、投票按钮、菜单按钮、回复按钮
- 显示/隐藏回复按钮、加载更多按钮  
- Feed 操作按钮（点赞、评论、分享）
- 对话框按钮（提交、取消、保存）
- 信息流帖子中的动态任务按钮

有关详细的主题化文档，请参见 [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).