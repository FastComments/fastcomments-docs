[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments demandera seulement à l'utilisateur son commentaire, son nom d'utilisateur et son courriel.

Cependant, dans certaines situations, vous pouvez vouloir que l'utilisateur laisse un lien vers son propre blog ou site Web.

Nous pouvons activer l'affichage d'un champ de saisie supplémentaire pour que l'utilisateur puisse indiquer l'URL de son site Web en définissant le drapeau **enableCommenterLinks** sur true:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

Lorsque cette URL est fournie, le compte de l'utilisateur sera mis à jour et le nom d'utilisateur associé à tous ses commentaires, passés et futurs, pointera vers cette URL.

Ceci peut être personnalisé sans code, sur la page de personnalisation du widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]

---