Всички бутони и елементи от потребителския интерфейс в FastComments SDK поддържат теми. Използвайте `FastCommentsTheme.Builder` за пълен контрол върху брандирането на вашето приложение.

### Програмно темизиране (Препоръчително)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Бутоните за действие: Изпрати, гласуване, меню, харесване/споделяне
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Бутоните за отговор: Бутоните за отговор на коментар  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Превключващи бутони: Покажи/скрий отговори
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Бутоните за "зареди повече": Бутони за пагинация
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Приложете темата
sdk.setTheme(theme)
```

### Бързо презаписване на цветовете

Презапишете цветови ресурси във вашия `colors.xml` за лесно брандиране:

```xml
<!-- В res/values/colors.xml на вашето приложение -->
<resources>
    <!-- Променете всички основни елементи на UI -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Или персонализирайте специфични типове бутони -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Покритие на тематизираните бутони

**Всеки бутон в SDK поддържа теми:**
- Бутоните за изпращане, бутоните за гласуване, бутоните на менюто, бутоните за отговор
- Бутоните за показване/скриване на отговори, бутоните 'зареди повече'  
- Бутоните за действия във фийда (харесване, коментар, споделяне)
- Бутоните в диалози (изпрати, отмени, запази)
- Динамични бутони за задачи в публикациите във фийда

За подробна документация относно темите, вижте [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).