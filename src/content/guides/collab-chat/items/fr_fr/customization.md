### Prise en charge du mode sombre

### Mode sombre dynamique

Si le mode sombre de votre site est activé en ajoutant une classe `.dark` à l'élément body, l'interface Collab Chat respectera automatiquement ce réglage sans nécessiter de réinitialisation. Les styles du widget sont conçus pour réagir à la présence de cette classe.

[inline-code-attrs-start title = 'Exemple CSS pour le mode sombre'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Votre CSS pour le mode sombre */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Personnalisation par CSS

Vous pouvez personnaliser l'apparence des surlignages, des fenêtres de chat et d'autres éléments à l'aide de CSS. Le widget ajoute des classes spécifiques que vous pouvez cibler dans votre feuille de style.

Les surlignages de texte utilisent le système de style des bulles de commentaire de FastComments, donc toutes les personnalisations que vous avez appliquées au widget de commentaires standard affecteront également Collab Chat.

### Personnalisation de la barre supérieure

La barre supérieure affiche le nombre d'utilisateurs en ligne et le nombre de discussions. Vous pouvez personnaliser sa position en fournissant un élément personnalisé comme `topBarTarget` :

[inline-code-attrs-start title = 'Emplacement personnalisé de la barre supérieure'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Ou la désactiver complètement en la définissant sur `null` :

[inline-code-attrs-start title = 'Désactiver la barre supérieure'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Comportement sur mobile

Sur les écrans de moins de 768px de large, Collab Chat bascule automatiquement vers une mise en page optimisée pour mobile. Les fenêtres de chat apparaissent en plein écran au lieu de flotter à côté du texte, et le délai de sélection est supprimé pour une interaction plus immédiate.

Ce comportement est intégré et ne nécessite aucune configuration. Le widget détecte automatiquement la taille de l'écran et s'ajuste en conséquence.

### Apparence de la fenêtre de chat

Les fenêtres de chat font 410px de large sur desktop avec une flèche de 16px pointant vers le texte surligné. Les fenêtres se positionnent automatiquement en fonction de l'espace disponible dans la fenêtre d'affichage, en utilisant des classes de positionnement comme `to-right`, `to-left`, `to-top`, et `to-bottom`.

Vous pouvez ajouter du CSS personnalisé pour ajuster les couleurs, les polices, les espacements ou d'autres propriétés visuelles de ces fenêtres. Les fenêtres de chat utilisent la même structure de composants que le widget FastComments standard, elles héritent donc de toutes les personnalisations globales que vous avez appliquées.

### Localisation

Collab Chat prend en charge toutes les mêmes options de localisation que le widget FastComments standard. Définissez l'option `locale` pour afficher le texte de l'interface dans différentes langues :

[inline-code-attrs-start title = 'Définir la locale'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Espagnol
});
[inline-code-end]

FastComments prend en charge des dizaines de langues. Le paramètre locale affecte tout le texte de l'interface utilisateur, y compris les invites, les boutons et le texte des champs de saisie.

### Options de personnalisation héritées

Puisque Collab Chat étend le widget de commentaires standard, il hérite de toutes les options de personnalisation du widget de base. Cela inclut les classes CSS personnalisées, les traductions personnalisées, la personnalisation des avatars, le formatage des dates, et bien plus encore.

Consultez la documentation principale de personnalisation de FastComments pour la liste complète des options de personnalisation disponibles.

### Utilisation de polices personnalisées

Si votre site utilise des polices personnalisées, l'interface Collab Chat héritera de ces polices depuis le CSS de votre page. Il se peut que vous deviez créer une règle de personnalisation du widget et `@import` toutes les polices dans le CSS personnalisé de cette règle si vous
souhaitez que la fenêtre de chat flottante utilise les mêmes polices.