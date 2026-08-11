[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments ne demandera à l'utilisateur que son commentaire, son nom d'utilisateur et son e‑mail.

Cependant, dans certaines situations, vous pouvez souhaiter que l'utilisateur laisse un lien vers son propre blog ou site web.

Nous pouvons activer l'affichage d'un champ de saisie supplémentaire pour laisser l'URL du site web de l'utilisateur en définissant le drapeau **enableCommenterLinks** sur true :

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Activation des liens du commentateur'; code-example-end]

Lorsque l'URL est fournie, le compte de l'utilisateur sera mis à jour et tous ses noms d'utilisateur sur tous les commentaires passés et futurs seront liés à cette URL.

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='Page de personnalisation du widget avec la case à cocher des liens du commentateur activée pour ajouter un champ d\'URL de site web au formulaire de commentaire'; title='Activation des liens du commentateur' app-screenshot-end]