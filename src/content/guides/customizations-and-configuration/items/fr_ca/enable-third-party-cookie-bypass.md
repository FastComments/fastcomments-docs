---
[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Pour l'authentification, FastComments dépend de l'activation des cookies tiers dans votre navigateur. Sans ceux-ci, les utilisateurs devront toujours laisser leur courriel pour commenter (à moins que le champ de saisie du courriel soit masqué), et leurs commentaires apparaîtront toujours comme non vérifiés (par défaut).

Pour y remédier, vous pouvez activer le contournement des cookies tiers.

Lorsque ce paramètre est activé, il provoque une petite fenêtre contextuelle indiquant que l'utilisateur est en cours de connexion. Cette fenêtre s'affiche chaque fois que l'utilisateur interagit avec le widget de commentaires ; par exemple, lorsqu'il publie un commentaire.

Nous pouvons le faire dans le code en définissant l'indicateur **enableThirdPartyCookieBypass** sur true :

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Activation du contournement des cookies tiers'; code-example-end]

Nous pouvons aussi configurer cela via l'interface de personnalisation du widget, sous `Enable Third-Party Cookie Popup` :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Activation du contournement des cookies tiers' app-screenshot-end]

---