Svi gumbi i UI elementi u FastComments SDK-u su prilagodljivi temom. Koristite `FastCommentsTheme.Builder` za potpunu kontrolu nad brendiranjem vaše aplikacije.

### Programsko prilagođavanje teme (preporučeno)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Akcijski gumbi: Pošalji, glasaj, izbornik, gumbi sviđanja/dijeljenja
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Gumbi za odgovor: Gumbi za odgovaranje na komentare  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Preklopni gumbi: Gumbi za prikaz/sakrij odgovore
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Gumbi 'učitaj više': gumbi za paginaciju
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Primijeni temu
sdk.setTheme(theme)
```

### Brzo prepisivanje boja

Prepišite resurse boja u svom `colors.xml` za jednostavno brendiranje:

```xml
<!-- U res/values/colors.xml vaše aplikacije -->
<resources>
    <!-- Promijenite sve primarne UI elemente -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Ili prilagodite određene tipove gumba -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Pokrivenost temiranih gumba

**Svaki gumb u SDK-u podržava prilagodbu teme:**
- Gumbi Pošalji, gumbi za glasanje, gumbi izbornika, gumbi za odgovor
- Gumbi za prikaz/sakrij odgovore, gumbi 'učitaj više'  
- Akcijski gumbi u feedu (sviđa mi se, komentar, dijeli)
- Dialog gumbi (pošalji, otkaži, spremi)
- Dinamički gumbi zadataka u objavama feeda

Za detaljnu dokumentaciju o temiranju, pogledajte [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).