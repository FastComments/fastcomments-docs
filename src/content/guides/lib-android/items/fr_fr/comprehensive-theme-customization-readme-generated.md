Tous les boutons et éléments d'interface du SDK FastComments sont personnalisables par thème. Utilisez `FastCommentsTheme.Builder` pour un contrôle complet du branding de votre application.

### Thématisation programmatique (recommandé)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Boutons d'action : Boutons Envoyer, vote, menu, aimer/partager
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Boutons de réponse : Boutons de réponse aux commentaires  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Boutons bascule : Boutons afficher/masquer les réponses
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Boutons charger plus : Boutons de pagination
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Appliquer le thème
sdk.setTheme(theme)
```

### Remplacement rapide des couleurs

Override color resources in your `colors.xml` for simple branding:

```xml
<!-- Dans le res/values/colors.xml de votre application -->
<resources>
    <!-- Modifiez tous les éléments d'interface principaux -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Ou personnalisez des types de boutons spécifiques -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Couverture des boutons thématisés

**Chaque bouton du SDK prend en charge la thématisation :**
- Boutons d'envoi, boutons de vote, boutons de menu, boutons de réponse
- Boutons afficher/masquer les réponses, boutons charger plus  
- Boutons d'action du fil (aimer, commenter, partager)
- Boutons de dialogue (envoyer, annuler, enregistrer)
- Boutons de tâche dynamiques dans les publications du fil

Pour une documentation détaillée sur la thématisation, consultez [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).