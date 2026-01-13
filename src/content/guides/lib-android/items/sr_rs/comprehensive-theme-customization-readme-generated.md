Сви дугмад и елементи корисничког интерфејса у FastComments SDK-у су прилагодљиви темом. Користите `FastCommentsTheme.Builder` за потпуну контролу над брендирањем ваше апликације.

### Програмирано темење (Препоручено)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Акциона дугмад: Пошаљи, дугмад за гласање, мени, дугмад за лајк/дељење
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Дугмад за одговор: дугмад за одговор на коментаре  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Дугмад за пребацивање: дугмад за приказ/скривање одговора
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Дугмад 'Учитај више': дугмад за пагинацију
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Примени тему
sdk.setTheme(theme)
```

### Брза промена боја

Замените ресурсе боја у вашем `colors.xml` ради једноставног брендирања:

```xml
<!-- У res/values/colors.xml ваше апликације -->
<resources>
    <!-- Промените све примарне UI елементе -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Или прилагодите специфичне типове дугмади -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Покривеност дугмади у теми

**Сва дугмад у SDK-у подржавају теме:**
- Дугмад за слање, дугмад за гласање, дугмад менија, дугмад за одговор
- Дугмад за приказ/скривање одговора, дугмад 'Учитај више'  
- Дугмад радњи у фиду (лајк, коментар, дељење)
- Дугмад у дијалозима (пошаљи, откажи, сачувај)
- Динамичка дугмад за задатке у постовима фида

За детаљну документацију о темама, погледајте [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).