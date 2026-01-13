All buttons and UI elements in the FastComments SDK are themeable. Use the `FastCommentsTheme.Builder` for complete control over your app's branding.

### Programmatic Theming (Recommended)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Action buttons: Send, vote, menu, like/share buttons
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Reply buttons: Comment reply buttons  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Toggle buttons: Show/hide replies buttons
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Load more buttons: Pagination buttons
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Apply the theme
sdk.setTheme(theme)
```

### Quick Color Override

Override color resources in your `colors.xml` for simple branding:

```xml
<!-- In your app's res/values/colors.xml -->
<resources>
    <!-- Change all primary UI elements -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Or customize specific button types -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Themed Button Coverage

**Every button in the SDK supports theming:**
- Send buttons, vote buttons, menu buttons, reply buttons
- Show/hide replies buttons, load more buttons  
- Feed action buttons (like, comment, share)
- Dialog buttons (submit, cancel, save)
- Dynamic task buttons in feed posts

For detailed theming documentation, see [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).