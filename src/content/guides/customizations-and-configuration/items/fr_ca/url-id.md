[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Lorsque le fil de commentaires est affiché, ou lorsqu'un commentaire est publié, FastComments doit savoir à quelle page, quel article ou quel produit ces commentaires appartiennent.

Pour ce faire, nous utilisons ce que nous appelons le "URL ID". Il s'agit soit d'un identifiant, comme une chaîne ou un nombre, soit d'une URL.

Par défaut, si vous ne spécifiez pas le urlId, il sera défini sur l'URL de la page. Nous prendrons l'URL courante et la nettoierons pour supprimer les paramètres marketing courants ou les identifiants de suivi.

Dans le cas d'intégrations tierces, comme WordPress, notre plugin utilisera généralement l'identifiant qui représente l'élément affiché comme URL ID, par exemple l'id de l'article/de la page.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Un élément que nous mentionnerons souvent dans ce document est l'<a href="https://fastcomments.com/auth/my-account/customize-widget/new">interface de personnalisation du widget</a>.

Cette interface permet d'apporter de nombreuses modifications au widget de commentaires sans écrire de code.

Lors de la création d'une règle de personnalisation, nous souhaitons souvent qu'elle s'applique à toutes les pages de notre site. Toutefois, dans certains cas, nous voulons personnaliser le widget de commentaires sur une page particulière, soit pour appliquer un style personnalisé, soit pour rendre les commentaires de cette page anonymes. Vous pouvez aussi, par exemple, faire apparaître immédiatement les commentaires en direct sur certaines pages, tout en les masquant sous des boutons de notification sur d'autres.

Tout cela est possible via le champ URL ID sur cette page, qui ressemble à ceci :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

La valeur de ce champ doit correspondre au paramètre *urlId* transmis au widget de commentaires. Si vous souhaitez que votre règle de personnalisation soit indépendante du *urlId*, laissez ce champ vide ou saisissez *.

As of 2023 the `URL ID` field in widget customization now also takes patterns! For example you may
have `*/blog/*` to add styling specific to your blog and `*/store/*` to have styling specific to your store,
all while using the same domain.

### Pièges

1. Si votre page contient des paramètres de fragment (par exemple example.com#page-1) - ceux-ci feront partie du URL ID, par défaut.
2. Pendant les migrations, par exemple de WordPress vers Gatsby, il se peut que vous deviez migrer les valeurs de URL ID des commentaires après la migration initiale. Pour cela, contactez‑nous.