Сва дугмад и елементи корисничког интерфејса у FastComments SDK-у су прилагодљиви темом. Користите `FastCommentsTheme.Builder` за потпуну контролу брендирања ваше апликације.

### Програмско прилагођавање теме (Препоручено)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Акционa дугмад: Пошаљи, гласај, мени, лајк/пошаљи дугмад
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Дугмад за одговор: Дугмад за одговор на коментар  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Прекидачка дугмад: Дугмад за приказ/сакривање одговора
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Дугмад 'Учитај више': Дугмад за пагинацију
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Примени тему
sdk.setTheme(theme)
```

### Брзо прилагођавање боја

Преопишите ресурсе боја у вашем `colors.xml` за једноставно брендирање:

```xml
<!-- У res/values/colors.xml ваше апликације -->
<resources>
    <!-- Промените све примарне елементе корисничког интерфејса -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Или прилагодите специфичне типове дугмади -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Обухват тематизованих дугмади

**Свако дугме у SDK-у подржава прилагођавање теме:**
- Дугмад за слање, дугмад за гласање, дугмад менија, дугмад за одговор
- Дугмад за приказ/сакривање одговора, дугмад 'Учитај више'  
- Акциона дугмад у фиду (лајк, коментар, дељење)
- Дугмад у дијалогу (потврди, откажи, сачувај)
- Динамичка дугмад задатака у постовима фида

За детаљну документацију о тематизацији, погледајте [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).