[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments permet aux utilisateurs de bloquer d'autres utilisateurs. Le blocage d'un utilisateur fera en sorte que ses commentaires
soient masqués, empêchera les notifications entre les utilisateurs, et ainsi de suite.

Il peut être souhaitable de désactiver cette fonctionnalité. Cela peut se faire de la manière suivante :

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

Cela peut également être fait sans code, ce qui active également une validation côté serveur appropriée, via l'interface de personnalisation du Widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]