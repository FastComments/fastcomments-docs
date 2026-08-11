[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments permet aux utilisateurs de bloquer d'autres utilisateurs. Bloquer un utilisateur masquera ses commentaires, empêche les notifications entre les utilisateurs, etc.

Il peut être souhaitable de désactiver cette fonctionnalité. Cela peut être fait ainsi :

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Cela peut également être fait sans code, ce qui active également une validation côté serveur appropriée, via l'interface de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='Option de désactivation du blocage dans l\'interface de personnalisation du widget, qui empêche les utilisateurs de se bloquer mutuellement'; title='Désactiver le blocage' app-screenshot-end]