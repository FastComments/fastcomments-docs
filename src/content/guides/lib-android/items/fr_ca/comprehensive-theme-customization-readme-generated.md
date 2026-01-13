Tous les boutons et éléments de l'interface utilisateur du SDK FastComments sont personnalisables par thème. Utilisez le `FastCommentsTheme.Builder` pour un contrôle complet de l'image de marque de votre application.

### Thématisation par programmation (recommandé)

```kotlin
val theme = FastCommentsTheme.Builder()
    // Boutons d'action : Envoyer, voter, menu, boutons J'aime/Partager
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // Boutons de réponse : Boutons de réponse aux commentaires  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // Boutons bascule : Boutons Afficher/masquer les réponses
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // Boutons 'charger plus' : Boutons de pagination
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// Appliquer le thème
sdk.setTheme(theme)
```

### Remplacement rapide des couleurs

Remplacez les ressources de couleur dans votre `colors.xml` pour une personnalisation simple de la marque :

```xml
<!-- Dans le res/values/colors.xml de votre application -->
<resources>
    <!-- Changer tous les éléments d'interface principaux -->
    <color name="primary">#FF1976D2</color>
    
    <!-- Ou personnaliser des types de boutons spécifiques -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### Portée des boutons thématisés

**Tous les boutons du SDK prennent en charge la thématisation :**
- Boutons d'envoi, boutons de vote, boutons de menu, boutons de réponse
- Boutons Afficher/masquer les réponses, boutons 'charger plus'  
- Boutons d'action du flux (j'aime, commentaire, partage)
- Boutons de dialogue (soumettre, annuler, enregistrer)
- Boutons de tâche dynamiques dans les publications du flux

Pour la documentation détaillée sur la thématisation, consultez [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md).