[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments est conçu pour être personnalisable, et la police utilisée par ses widgets ne fait pas exception.

Par défaut, FastComments utilise la `system font stack` pour s'afficher au mieux sur un large éventail d'appareils.

Pour définir vos propres polices, consultez la [Documentation sur le CSS personnalisé](/guide-customizations-and-configuration.html#custom-css).

Vous y trouverez une méthode pour définir du CSS personnalisé, ce qui vous permettra de définir les polices de votre choix.

#### Comment définir la police

Pour remplacer la police, nous recommandons de définir votre CSS en utilisant les sélecteurs `.fast-comments, textarea`. Par exemple :

[inline-code-attrs-start title = 'Exemple de police externe personnalisée'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]