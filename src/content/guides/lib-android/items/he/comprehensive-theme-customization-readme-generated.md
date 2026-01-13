כל הכפתורים ואלמנטים בממשק המשתמש בערכת הפיתוח של FastComments ניתנים לעיצוב. השתמש ב- `FastCommentsTheme.Builder` לשליטה מלאה במיתוג של האפליקציה שלך.

### עיצוב תכנותי (מומלץ)

```kotlin
val theme = FastCommentsTheme.Builder()
    // כפתורי פעולה: שליחה, הצבעה, תפריט, כפתורי לייק/שיתוף
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // כפתורי תגובה: כפתורי מענה להערות  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // כפתורי החלפה: כפתורי הצג/הסתר תגובות
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // כפתורי טעינת עוד: כפתורי עימוד
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// החלת העיצוב
sdk.setTheme(theme)
```

### החלפה מהירה של צבעים

Override color resources in your `colors.xml` for simple branding:

```xml
<!-- בקובץ res/values/colors.xml של האפליקציה שלך -->
<resources>
    <!-- שנה את כל רכיבי הממשק הראשיים -->
    <color name="primary">#FF1976D2</color>
    
    <!-- או התאם סוגי כפתורים ספציפיים -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### כיסוי כפתורים מעוצבים

**כל כפתור ב-SDK תומך בעיצוב:**
- כפתורי שליחה, כפתורי הצבעה, כפתורי תפריט, כפתורי תגובה
- כפתורי הצג/הסתר תגובות, כפתורי 'טעון עוד'  
- כפתורי פעולה בפיד (לייק, תגובה, שיתוף)
- כפתורי דיאלוג (שלח, בטל, שמור)
- כפתורי משימות דינמיים בפוסטים של הפיד

לתיעוד מפורט על ערכות עיצוב, ראו את [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).