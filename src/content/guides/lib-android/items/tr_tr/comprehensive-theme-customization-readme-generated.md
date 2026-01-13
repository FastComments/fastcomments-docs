FastComments SDK'deki tüm düğmeler ve kullanıcı arayüzü öğeleri temalandırılabilir. Uygulamanızın markalaması üzerinde tam kontrol için `FastCommentsTheme.Builder` kullanın.

### Programatik Temalandırma (Önerilir)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Eylem düğmeleri: Gönder, oy, menü, beğen/paylaş düğmeleri
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Yanıt düğmeleri: Yorum yanıt düğmeleri  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Aç/Kapa düğmeleri: Yanıtları göster/gizle düğmeleri
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Daha fazla yükle düğmeleri: Sayfalandırma düğmeleri
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Temayı uygula
sdk.setTheme(theme)
```

### Hızlı Renk Geçersiz Kılma

Override color resources in your `colors.xml` for simple branding:

```xml
<!-- Uygulamanızın res/values/colors.xml dosyasında -->
<resources>
    <!-- Tüm birincil kullanıcı arayüzü öğelerini değiştirin -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Veya belirli düğme türlerini özelleştirin -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Temalandırılmış Düğme Kapsamı

**SDK'deki her düğme temalandırılabilir:**
- Gönder düğmeleri, oy düğmeleri, menü düğmeleri, yanıt düğmeleri
- Yanıtları göster/gizle düğmeleri, daha fazla yükle düğmeleri  
- Akış eylem düğmeleri (beğen, yorum, paylaş)
- Diyalog düğmeleri (gönder, iptal, kaydet)
- Akış gönderilerindeki dinamik görev düğmeleri

Detaylı temalandırma dokümantasyonu için [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md) dosyasına bakın.