Усі кнопки та елементи UI в FastComments SDK підтримують темізацію. Використовуйте `FastCommentsTheme.Builder` для повного контролю над брендингом вашого додатка.

### Програмна темізація (рекомендовано)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Кнопки дій: Відправити, голосування, меню, кнопки вподобати/поділитися
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Кнопки відповіді: Кнопки відповіді на коментарі  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Перемикачі: Кнопки показати/сховати відповіді
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Кнопки завантажити ще: Кнопки пагінації
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Застосувати тему
sdk.setTheme(theme)
```

### Швидка заміна кольорів

Перевизначте ресурси кольорів у вашому `colors.xml` для простого брендингу:

```xml
<!-- У res/values/colors.xml вашого додатка -->
<resources>
    <!-- Змініть усі основні елементи UI -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Або налаштуйте певні типи кнопок -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Покриття темізації кнопок

**Кожна кнопка в SDK підтримує темізацію:**
- Кнопки відправлення, кнопки голосування, кнопки меню, кнопки відповіді
- Кнопки показати/сховати відповіді, кнопки завантажити ще  
- Кнопки дій стрічки (подобається, коментар, поділитися)
- Кнопки діалогів (відправити, скасувати, зберегти)
- Динамічні кнопки завдань у дописах стрічки

Детальну документацію з темізації див. у [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).