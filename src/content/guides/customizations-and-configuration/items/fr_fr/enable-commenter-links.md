[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments ne demandera à l'utilisateur que son commentaire, son nom d'utilisateur et son adresse e-mail.

Cependant, dans certains cas, vous pouvez souhaiter que l'utilisateur laisse un lien vers son propre blog ou site web.

Vous pouvez activer l'affichage d'un champ supplémentaire pour saisir l'URL du site web de l'utilisateur en définissant le flag **enableCommenterLinks** sur true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Lorsque cette URL est fournie, le compte de l'utilisateur sera mis à jour et le nom d'utilisateur affiché sur tous ses commentaires passés et futurs renverra vers cette URL.

Cela peut être personnalisé sans code, depuis la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]