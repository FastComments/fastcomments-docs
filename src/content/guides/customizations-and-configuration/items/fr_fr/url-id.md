[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Lors du rendu d'un fil de commentaires, ou lors de la rédaction d'un commentaire, FastComments doit savoir à quelle page, article ou produit ces commentaires appartiennent.

Pour ce faire, nous utilisons quelque chose que nous appelons le « URL ID ». C’est soit un identifiant, comme une chaîne ou un nombre, soit une URL.

Par défaut, si vous ne spécifiez pas le urlId, il deviendra l'URL de la page. Nous prendrons l'URL de la page actuelle et la nettoierons pour supprimer les paramètres marketing courants ou les identifiants de suivi.

Dans le cas d'intégrations tierces, comme WordPress, notre plugin utilisera généralement l'identifiant qui représente l'information actuellement affichée comme URL ID, par exemple l'ID de l'article ou de la page.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Définir un URL ID personnalisé'; code-example-end]

Une chose que nous référencerons souvent dans ce document est l'<a href="https://fastcomments.com/auth/my-account/customize-widget/new">interface de personnalisation du widget</a>.

Cette interface peut être utilisée pour apporter de nombreux changements au widget de commentaires sans écrire de code.

Lors de la création d'une règle de personnalisation, nous voulons souvent qu'elle s'applique à toutes les pages de notre site. Cependant, dans certains cas, nous souhaitons personnaliser le widget de commentaires sur une page particulière, soit pour appliquer un style personnalisé, soit peut‑être rendre les commentaires de cette page anonymes. Vous pourriez également, par exemple, faire apparaître les commentaires en direct immédiatement sur certaines pages, tout en les masquant sous des boutons de notification sur d'autres.

Tout cela est possible grâce au champ de saisie URL ID sur cette page, qui ressemble à ceci :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='Champ URL ID utilisé pour appliquer une règle de personnalisation à une page, ou à un motif tel que */blog/*'; title='Champ de saisie URL ID dans la page de personnalisation du widget' app-screenshot-end]

La valeur de ce champ doit correspondre au paramètre *urlId* passé au widget de commentaires. Si vous souhaitez que votre règle de personnalisation soit indépendante du *urlId*, laissez ce champ vide ou saisissez *.

Depuis 2023, le champ `URL ID` dans la personnalisation du widget accepte également des motifs ! Par exemple, vous pouvez avoir `*/blog/*` pour ajouter un style spécifique à votre blog et `*/store/*` pour un style spécifique à votre boutique, tout en utilisant le même domaine.

### Pièges

1. Si votre page possède des paramètres de hachage (comme example.com#page-1) – cela fera partie du URL ID, par défaut.  
2. Lors des migrations, par exemple de WordPress vers Gatsby, vous devrez peut‑être migrer les valeurs de commentaires URL ID après la migration initiale. Pour cela, contactez‑nous.

---