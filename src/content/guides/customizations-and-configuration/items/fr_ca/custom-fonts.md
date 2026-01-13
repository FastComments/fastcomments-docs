[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments est conçu pour être personnalisé, et la police utilisée par nos widgets n'en fait pas exception.

Par défaut, FastComments utilise la `system font stack` pour offrir le meilleur rendu possible sur une grande variété d'appareils.

Pour définir vos propres polices, consultez la [documentation CSS personnalisée](/guide-customizations-and-configuration.html#custom-css).

Vous y trouverez une méthode pour définir du CSS personnalisé, ce qui vous permettra de spécifier les polices que vous souhaitez.

#### Comment définir la police

Pour remplacer la police, nous vous recommandons de définir votre CSS en utilisant les sélecteurs `.fast-comments, textarea`. Par exemple :

[inline-code-attrs-start title = 'Exemple de police externe personnalisée'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300&display=swap');
.fast-comments, textarea {
    font-family: 'Roboto', sans-serif;
}
[inline-code-end]

---