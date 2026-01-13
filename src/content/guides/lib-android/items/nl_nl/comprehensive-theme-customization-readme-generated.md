Alle knoppen en UI-elementen in de FastComments SDK zijn te thematiseren. Gebruik de `FastCommentsTheme.Builder` voor volledige controle over de huisstijl van uw app.

### Thematisering via code (Aanbevolen)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Actieknoppen: verzenden, stemmen, menu, like/delen-knoppen
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Antwoordknoppen: knoppen voor reageren op opmerkingen  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Wisselknoppen: Knoppen om reacties te tonen/verbergen
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Laad meer-knoppen: paginatieknoppen
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Pas het thema toe
sdk.setTheme(theme)
```

### Snelle kleuraanpassing

Overschrijf kleurbronnen in uw `colors.xml` voor eenvoudige aanpassing van de huisstijl:

```xml
<!-- In het res/values/colors.xml van uw app -->
<resources>
    <!-- Wijzig alle primaire UI-elementen -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Of pas specifieke knopsoorten aan -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Knoppen die door het thema worden gedekt

**Elke knop in de SDK ondersteunt thematisering:**
- Verzendknoppen, stemknoppen, menuknoppen, antwoordknoppen
- Knoppen voor tonen/verbergen van reacties, 'laad meer'-knoppen  
- Actieknoppen in de feed (vind-ik-leuk, reageren, delen)
- Dialoogknoppen (verzenden, annuleren, opslaan)
- Dynamische taakknoppen in feedposts

Voor gedetailleerde themadocumentatie, zie [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).