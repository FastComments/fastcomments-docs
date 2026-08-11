[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Pour l'authentification, FastComments dépend de l'activation des cookies tiers dans votre navigateur. Sans eux, les utilisateurs devront toujours
laisser leur e‑mail pour commenter (à moins que le champ de saisie d'e‑mail soit masqué), et leurs commentaires apparaîtront toujours comme non vérifiés (par défaut).

Pour contourner cela, vous pouvez activer le contournement des cookies tiers. 

Lorsque ce paramètre est activé, il déclenchera une petite fenêtre contextuelle affichant un message indiquant que l'utilisateur est en cours de connexion. Cette fenêtre
apparaît chaque fois que l'utilisateur interagit avec le widget de commentaires ; par exemple, s'il laisse un commentaire.

Nous pouvons faire cela dans le code en définissant le drapeau **enableThirdPartyCookieBypass** sur true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Activation du contournement des cookies tiers'; code-example-end]

Nous pouvons également configurer cela via l'interface de personnalisation du widget, sous `Enable Third-Party Cookie Popup` :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Page de personnalisation du widget avec la case à cocher Activer la fenêtre contextuelle des cookies tiers cochée'; title='Activation du contournement des cookies tiers' app-screenshot-end]

---