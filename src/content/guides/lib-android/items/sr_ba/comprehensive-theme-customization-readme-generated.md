Сва дугмад и UI елементи у FastComments SDK-у подложни су темама. Користите `FastCommentsTheme.Builder` за потпуну контролу над брендом ваше апликације.

### Програмско подешавање теме (препоручено)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Дугмад за акције: Поšаљи, гласање, мени, дугмад свиđa ми се/дијели
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Дугмад за одговор: Дугмад за одговарање на коментар  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Прекидачка дугмад: Дугмад за приказ/скривање одговора
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Дугмад за учитавање више: Дугмад за пагинацију
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Примјени тему
sdk.setTheme(theme)
```

### Брзо подешавање боја

Оверрајту ресурсе боја у вашем `colors.xml` за једноставно брендирање:

```xml
<!-- У вашој апликацији res/values/colors.xml -->
<resources>
    <!-- Промијените све примарне UI елементе -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Или прилагодите специфичне типове дугмади -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Обухват темatsких дугмади

**Свако дугме у SDK-у подржава тематизацију:**
- Дугмад за слање, дугмад за гласање, дугмад за мени, дугмад за одговор
- Дугмад за приказ/скривање одговора, дугмад за учитавање више  
- Дугмад радњи у фиду (свиđa ми се, коментар, дијели)
- Дугмад дијалога (пошаљи, откажи, саčувај)
- Динамичка дугмад задатака у објавама фида

За детаљну документацију о тематици, погледајте [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).