Vsi gumbi in elementi uporabniškega vmesnika v FastComments SDK so tematsko prilagodljivi. Uporabite `FastCommentsTheme.Builder` za popoln nadzor nad blagovno znamko vaše aplikacije.

### Programska tematska prilagoditev (priporočeno)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Gumbi za dejanja: Pošlji, glasuj, meni, všečkaj/deli
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Gumbi za odgovor: Gumbi za odgovor na komentarje  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Preklopni gumbi: Gumbi za prikaz/skrij odgovore
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Gumbi 'Naloži več': Gumbi za paginacijo
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Uporabi temo
sdk.setTheme(theme)
```

### Hitro prepisovanje barv

Prepišite barvne vire v datoteki `colors.xml` za enostavno prilagoditev blagovne znamke:

```xml
<!-- V datoteki res/values/colors.xml vaše aplikacije -->
<resources>
    <!-- Spremenite vse primarne elemente uporabniškega vmesnika -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Ali prilagodite posebne vrste gumbov -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Pokrivanje tematskih gumbov

**Vsak gumb v SDK podpira tematsko prilagajanje:**
- Gumbi za pošiljanje, gumbi za glasovanje, gumbi menija, gumbi za odgovore
- Gumbi za prikaz/skrij odgovore, gumbi 'naloži več'  
- Gumbi dejanj v feedu (všeč, komentar, deljenje)
- Gumbi dialogov (pošlji, prekliči, shrani)
- Dinamični gumbi opravil v objavah v feedu

Za podrobno dokumentacijo o tematskem prilagajanju glejte [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).