FastComments SDK में सभी बटन और UI तत्व थीमयोग्य हैं। अपने ऐप की ब्रांडिंग पर पूर्ण नियंत्रण के लिए `FastCommentsTheme.Builder` का उपयोग करें।

### प्रोग्रामेटिक थीमिंग (अनुशंसित)

```kotlin
val theme = FastCommentsTheme.Builder()
    // एक्शन बटन: Send, vote, menu, like/share बटन
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // रिप्लाई बटन: कमेंट रिप्लाई बटन  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // टॉगल बटन: Show/hide replies बटन
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Load more बटन: पेजिनेशन बटन
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// थीम लागू करें
sdk.setTheme(theme)
```

### त्वरित रंग ओवरराइड

सरल ब्रांडिंग के लिए अपने `colors.xml` में रंग संसाधनों को ओवरराइड करें:

```xml
<!-- अपने ऐप के res/values/colors.xml में -->
<resources>
    <!-- सभी प्राथमिक UI तत्व बदलें -->
    <color name="primary">#FF1976D2</color>
    
    <!-- या विशिष्ट बटन प्रकार अनुकूलित करें -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### थीम वाले बटन कवरेज

**SDK में हर बटन थीमिंग का समर्थन करता है:**
- Send बटन, vote बटन, menu बटन, reply बटन
- Show/hide replies बटन, load more बटन  
- फ़ीड एक्शन बटन (like, comment, share)
- डायलॉग बटन (submit, cancel, save)
- फ़ीड पोस्ट में डायनामिक टास्क बटन

विस्तृत थीमिंग दस्तावेज़ीकरण के लिए, [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md) देखें।