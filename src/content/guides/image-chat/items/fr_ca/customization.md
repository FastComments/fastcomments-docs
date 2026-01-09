### Prise en charge du mode sombre

Image Chat inclut une prise en charge intégrée du mode sombre. Lorsque vous définissez `hasDarkBackground: true` dans votre configuration, les fenêtres de discussion et les éléments de l'interface s'ajustent automatiquement pour bien fonctionner sur des arrière-plans sombres.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

Le style du mode sombre s'applique aux fenêtres de discussion, aux carrés marqueurs et à tous les éléments interactifs. Si votre site dispose d'un commutateur de mode sombre, vous pouvez réinitialiser le widget lorsque le mode change, ou utiliser l'approche par classe body décrite ci-dessous.

### Mode sombre dynamique

Si le mode sombre de votre site est contrôlé en ajoutant une classe `.dark` à l'élément body, l'interface d'Image Chat respectera automatiquement cela sans nécessiter de réinitialisation. Les styles du widget sont conçus pour répondre à la présence de cette classe.

```css
/* Votre CSS pour le mode sombre */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Personnalisation avec CSS

Vous pouvez personnaliser l'apparence des marqueurs, des fenêtres de discussion et d'autres éléments en utilisant du CSS. Le widget ajoute des classes spécifiques que vous pouvez cibler dans votre feuille de style.

Les carrés et les fenêtres de chat utilisent le système de style de bulles de commentaire de FastComments, donc toutes les personnalisations que vous avez appliquées au widget de commentaires standard affecteront également Image Chat.

### Taille des carrés de chat

L'option `chatSquarePercentage` contrôle la taille des marqueurs cliquables. La valeur par défaut est de 5 % de la largeur de l'image :

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Carrés plus grands et plus visibles
});
```

Des valeurs plus petites créent des marqueurs plus subtils qui se fondent dans l'image. Des valeurs plus grandes rendent les marqueurs plus proéminents et plus faciles à cliquer, notamment sur les appareils mobiles ou pour des raisons d'accessibilité.

### Comportement sur mobile

Sur les écrans de moins de 768px de large, Image Chat bascule automatiquement vers une mise en page optimisée pour mobile. Les fenêtres de discussion apparaissent en plein écran au lieu de flotter à côté des marqueurs, offrant une meilleure utilisabilité sur les petits écrans.

Les marqueurs restent visibles à leurs positions réactives sur l'image. Les utilisateurs peuvent taper n'importe quel marqueur pour ouvrir l'interface de discussion en plein écran. Ce comportement est intégré et ne nécessite aucune configuration.

### Apparence des fenêtres de discussion

Les fenêtres de discussion mesurent 300px de large sur bureau avec une flèche de 16px pointant vers le marqueur. Les fenêtres se positionnent automatiquement en fonction de l'espace disponible dans la fenêtre d'affichage, en utilisant des classes de positionnement comme `to-right`, `to-left`, `to-top` et `to-bottom`.

Vous pouvez ajouter du CSS personnalisé pour ajuster les couleurs, les polices, les espacements ou d'autres propriétés visuelles de ces fenêtres. Les fenêtres de discussion utilisent la même structure de composant que le widget FastComments standard, elles héritent donc de toutes les personnalisations globales que vous avez appliquées.

### Initialisation différée

Les fenêtres de discussion s'initialisent au survol pour les utilisateurs sur bureau ou immédiatement lorsqu'elles sont créées. Cela réduit la charge initiale en ne rendant l'interface de discussion que lorsque les utilisateurs interagissent réellement avec un marqueur.

L'initialisation différée se produit de manière transparente. Les utilisateurs ne remarquent aucun délai, mais le navigateur n'a pas besoin de rendre des dizaines de fenêtres de discussion cachées si vous avez de nombreux marqueurs sur une image.

### Localisation

Image Chat prend en charge toutes les mêmes options de localisation que le widget FastComments standard. Définissez l'option `locale` pour afficher le texte de l'interface dans différentes langues :

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Français
});
```

FastComments prend en charge des dizaines de langues. Le réglage de la locale affecte tout le texte de l'interface utilisateur, y compris les invites, les boutons et le texte des espaces réservés.

### Options de personnalisation héritées

Puisque Image Chat étend le widget de commentaires standard, il hérite de toutes les options de personnalisation du widget de base. Cela inclut les classes CSS personnalisées, les traductions personnalisées, la personnalisation des avatars, le formatage des dates, et bien plus encore.

Consultez la documentation principale de personnalisation de FastComments pour la liste complète des options de personnalisation disponibles.

### Utilisation de polices personnalisées

Si votre site utilise des polices personnalisées, l'interface d'Image Chat héritera de ces polices depuis le CSS de votre page. Les fenêtres de discussion sont rendues à l'intérieur du DOM de votre page et respectent vos paramètres typographiques existants.

Pour de meilleurs résultats, assurez-vous que vos polices personnalisées sont chargées avant d'initialiser Image Chat, ou acceptez qu'il puisse y avoir un bref flash de texte non stylisé pendant le chargement des polices.

### Design visuel des marqueurs

Les marqueurs carrés ont un design visuel subtil qui les rend remarquables sans dominer l'image. Vous pouvez personnaliser leur apparence avec du CSS si vous souhaitez un traitement visuel différent.

Les marqueurs incluent des états au survol qui fournissent un retour visuel lorsque les utilisateurs déplacent leur souris dessus. Sur les appareils tactiles, l'interaction par tapotement fournit un retour immédiat en ouvrant la fenêtre de discussion.