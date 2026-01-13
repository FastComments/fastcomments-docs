[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Par défaut, le widget FastComments s'ajustera verticalement pour afficher tous les commentaires visibles. La pagination se fait au moyen d'un bouton «Afficher la suite» à la fin de la page en cours, car nous avons constaté que cette interaction est la plus agréable pour la plupart des utilisateurs.

Cependant, il existe des cas où le défilement infini est préférable. Par exemple, nous utilisons cette fonctionnalité dans notre produit Stream Chat.

Nous pouvons masquer les boutons «Afficher la suite» et passer au défilement infini en définissant le drapeau **enableInfiniteScrolling** sur true :

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Cela nécessite également l'ajout de CSS personnalisé. Ajoutez du CSS personnalisé pour le sélecteur `.comments` afin d'activer le défilement, par exemple :

[inline-code-attrs-start title = 'Activer le défilement infini'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Un exemple complet fonctionnel serait :

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

Dans l'exemple ci-dessus, nous utilisons la propriété `customCSS`; toutefois, il est conseillé d'utiliser plutôt l'interface de configuration du widget pour des raisons de performance. [Voir la documentation sur le CSS personnalisé.](/guide-customizations-and-configuration.html#custom-css)

---