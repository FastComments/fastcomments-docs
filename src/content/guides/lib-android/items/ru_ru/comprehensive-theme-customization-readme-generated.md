Все кнопки и элементы интерфейса в FastComments SDK поддерживают темизацию. Используйте `FastCommentsTheme.Builder` для полного контроля над брендингом вашего приложения.

### Программная темизация (рекомендуется)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Кнопки действий: отправить, голосовать, меню, кнопки «лайк/поделиться»
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Кнопки ответа: кнопки ответа на комментарий  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Переключающие кнопки: показать/скрыть ответы
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Кнопки «Загрузить ещё»: кнопки пагинации
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Применить тему
sdk.setTheme(theme)
```

### Быстрое переопределение цвета

Переопределите цветовые ресурсы в вашем `colors.xml` для простого брендирования:

```xml
<!-- В res/values/colors.xml вашего приложения -->
<resources>
    <!-- Изменить все основные элементы интерфейса -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Или настроить конкретные типы кнопок -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Охват темизации кнопок

**Каждая кнопка в SDK поддерживает темизацию:**
- Кнопки отправки, голосования, меню, кнопки ответа
- Кнопки показать/скрыть ответы, кнопки «Загрузить ещё»  
- Кнопки действий в ленте (лайк, комментарий, поделиться)
- Кнопки в диалогах (отправить, отмена, сохранить)
- Динамические кнопки задач в постах ленты

Для подробной документации по темизации смотрите [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).