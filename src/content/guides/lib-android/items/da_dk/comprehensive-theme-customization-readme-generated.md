Alle knapper og UI-elementer i FastComments SDK kan tematiseres. Brug `FastCommentsTheme.Builder` for fuld kontrol over din apps branding.

### Programmatisk temaindstilling (Anbefalet)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Handlingsknapper: Send, stemmeknapper, menu, like/del-knapper
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Svarknapper: Knapper til at svare på kommentarer  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Skiftknapper: Vis/skjul svar-knapper
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Indlæs flere-knapper: Paginering knapper
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Anvend temaet
sdk.setTheme(theme)
```

### Hurtig overskrivning af farver

Overskriv farveressourcer i din `colors.xml` for enkel branding:

```xml
<!-- I din apps res/values/colors.xml -->
<resources>
    <!-- Ændr alle primære UI-elementer -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Eller tilpas specifikke knap-typer -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Dækning af tematiserede knapper

**Hver knap i SDK'et understøtter temaindstilling:**
- Send-knapper, stemmeknapper, menuknapper, svarknapper
- Vis/skjul svar-knapper, indlæs flere-knapper  
- Feed-handlingsknapper (like, kommentar, del)
- Dialogknapper (send, annuller, gem)
- Dynamiske opgaveknapper i feedindlæg

For detaljeret dokumentation om temaindstilling, se [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).