Sva dugmad i UI elementi u FastComments SDK-u podržavaju teme. Koristite `FastCommentsTheme.Builder` za potpunu kontrolu nad brendingom vaše aplikacije.

### Programsko podešavanje tema (preporučeno)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Dugmad za akcije: Pošalji, glasanje, meni, lajk/podeli dugmad
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Dugmad za odgovore: Dugmad za odgovore na komentare  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Toggle dugmad: Dugmad za prikaz/sakrij odgovore
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Dugmad za učitavanje više: Dugmad za paginaciju
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Primeni temu
sdk.setTheme(theme)
```

### Brzo prepisivanje boja

Prepišite resurse boja u vašem `colors.xml` za jednostavno brendiranje:

```xml
<!-- U res/values/colors.xml vaše aplikacije -->
<resources>
    <!-- Promenite sve primarne UI elemente -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Ili prilagodite specifične tipove dugmadi -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Obuhvat temiranih dugmadi

**Svako dugme u SDK-u podržava teme:**
- Dugmad za slanje, dugmad za glasanje, dugmad menija, dugmad za odgovore
- Dugmad za prikaz/sakrij odgovore, dugmad za učitavanje više  
- Dugmad za akcije u feedu (lajk, komentar, podeli)
- Dugmad u dijalozima (potvrdi, otkaži, sačuvaj)
- Dinamička dugmad zadataka u objavama feeda

Za detaljnu dokumentaciju o temama, pogledajte [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).