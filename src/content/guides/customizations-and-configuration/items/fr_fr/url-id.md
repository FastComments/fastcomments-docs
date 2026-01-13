[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Lors du rendu d'un fil de commentaires, ou lorsque vous laissez un commentaire, FastComments doit savoir à quelle page, article ou produit
ces commentaires appartiennent.

Pour cela, nous utilisons ce que nous appelons le "URL ID". Il s'agit soit d'un identifiant, comme une chaîne ou un nombre, soit d'une URL.

Par défaut, si vous ne spécifiez pas le urlId, il deviendra l'URL de la page. Nous prendrons l'URL actuelle de la page et la nettoierons pour supprimer
les paramètres marketing courants ou les identifiants de suivi.

Dans le cas des intégrations tierces, comme WordPress, notre plugin utilisera généralement l'identifiant représentant l'information consultée comme
le URL ID, par exemple l'id de l'article/de la page.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Un élément auquel nous faisons souvent référence dans ce document est l'<a href="https://fastcomments.com/auth/my-account/customize-widget/new">Interface de personnalisation du widget</a>.

Cette interface peut être utilisée pour effectuer de nombreuses modifications du widget de commentaires sans écrire de code.

Lorsque vous créez une règle de personnalisation, nous souhaitons souvent qu'elle s'applique à toutes les pages de notre site. Cependant, dans certains cas, nous voulons personnaliser le widget de commentaires
sur une page particulière, soit pour appliquer un style personnalisé, soit peut-être pour rendre les commentaires de cette page particuliers anonymes. Vous pouvez aussi, par exemple, afficher les commentaires en direct
immédiatement sur certaines pages, tout en les masquant sous des boutons de notification sur d'autres.

Tout cela est possible via le champ URL ID sur cette page, qui ressemble à ceci :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

La valeur de ce champ doit correspondre au paramètre *urlId* passé au widget de commentaires. Si vous souhaitez que votre règle de personnalisation soit indifférente au *urlId*, laissez ce champ vide ou saisissez *.

Depuis 2023, le champ `URL ID` dans la personnalisation du widget accepte désormais des motifs ! Par exemple, vous pouvez
avoir `*/blog/*` pour ajouter un style spécifique à votre blog et `*/store/*` pour un style spécifique à votre boutique,
tout en utilisant le même domaine.

### Points importants

1. Si votre page possède des paramètres de hachage (comme example.com#page-1) - ceux-ci feront partie du URL ID, par défaut.
2. Lors des migrations, par exemple de WordPress vers Gatsby, il se peut que vous deviez migrer les valeurs URL ID des commentaires après la migration initiale. Pour cela, contactez-nous.

---