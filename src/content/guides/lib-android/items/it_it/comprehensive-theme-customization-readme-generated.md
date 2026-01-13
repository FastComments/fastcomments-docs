Tutti i pulsanti e gli elementi UI nell'SDK FastComments sono personalizzabili con temi. Usa `FastCommentsTheme.Builder` per avere il controllo completo sul branding della tua app.

### Theming programmabile (Consigliato)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Pulsanti di azione: Invia, vota, menu, pulsanti mi piace/condividi
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Pulsanti di risposta: pulsanti per rispondere ai commenti  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Pulsanti toggle: mostra/nascondi risposte
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Pulsanti Carica altri: pulsanti di paginazione
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Applica il tema
sdk.setTheme(theme)
```

### Sovrascrittura rapida dei colori

Sovrascrivi le risorse di colore nel tuo `colors.xml` per un branding semplice:

```xml
<!-- Nel file res/values/colors.xml della tua app -->
<resources>
    <!-- Cambia tutti gli elementi UI primari -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Oppure personalizza tipi di pulsanti specifici -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Copertura dei pulsanti tematizzati

**Tutti i pulsanti nello SDK supportano i temi:**
- Pulsanti di invio, pulsanti di voto, pulsanti del menu, pulsanti di risposta
- Pulsanti mostra/nascondi risposte, pulsanti carica altri  
- Pulsanti di azione del feed (mi piace, commenta, condividi)
- Pulsanti del dialogo (invia, annulla, salva)
- Pulsanti di attività dinamiche nei post del feed

Per la documentazione dettagliata sul theming, consulta [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).