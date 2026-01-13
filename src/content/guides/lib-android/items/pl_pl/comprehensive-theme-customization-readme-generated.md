Wszystkie przyciski i elementy interfejsu użytkownika w SDK FastComments obsługują motywy. Użyj `FastCommentsTheme.Builder`, aby w pełni kontrolować branding swojej aplikacji.

### Programowe motywy (zalecane)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Przyciski akcji: wyślij, głosuj, menu, przyciski polubienia/udostępnienia
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Przyciski odpowiedzi: przyciski odpowiedzi w komentarzach  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Przełącznik: przyciski pokaż/ukryj odpowiedzi
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Przyciski 'pokaż więcej': przyciski paginacji
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Zastosuj motyw
sdk.setTheme(theme)
```

### Szybkie nadpisanie kolorów

Nadpisz zasoby kolorów w `colors.xml`, aby w prosty sposób dostosować branding:

```xml
<!-- W pliku res/values/colors.xml Twojej aplikacji -->
<resources>
    <!-- Zmień wszystkie podstawowe elementy interfejsu użytkownika -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Lub dostosuj konkretne typy przycisków -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Zakres przycisków objętych motywem

**Każdy przycisk w SDK obsługuje motywy:**
- Przyciski wysyłania, przyciski głosowania, przyciski menu, przyciski odpowiedzi
- Przyciski pokaż/ukryj odpowiedzi, przyciski 'pokaż więcej'  
- Przyciski akcji w kanale (polub, skomentuj, udostępnij)
- Przyciski w dialogach (wyślij, anuluj, zapisz)
- Dynamiczne przyciski zadań w postach kanału

Szczegółową dokumentację dotyczącą motywów znajdziesz w [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).