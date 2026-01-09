[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Par défaut, le widget FastComments redimensionnera sa hauteur pour afficher tous les commentaires visibles. La pagination se fait via un bouton "Afficher la suite" à la fin de la page en cours, car nous avons constaté que c'est l'interaction qui convient le mieux à la plupart des utilisateurs.

Cependant, il existe des cas où le défilement infini est préféré. Par exemple, nous utilisons cette fonctionnalité dans notre produit Stream Chat.

Nous pouvons masquer les boutons "Afficher la suite" et passer au défilement infini en définissant le drapeau **enableInfiniteScrolling** sur true :

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Cela nécessite également d'ajouter du CSS personnalisé. Ajoutez du CSS personnalisé pour le sélecteur `.comments` afin d'activer le défilement, par exemple :

[inline-code-attrs-start title = 'Activer le défilement infini'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Un exemple complet et fonctionnel serait :

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

Dans l'exemple ci‑dessus, nous utilisons la propriété `customCSS` ; cependant, il est recommandé d'utiliser l'interface de configuration du widget à la place pour des raisons de performance. [Voir la documentation sur le CSS personnalisé.](/guide-customizations-and-configuration.html#custom-css)