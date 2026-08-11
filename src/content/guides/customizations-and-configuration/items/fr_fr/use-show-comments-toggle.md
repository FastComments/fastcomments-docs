[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments affichera la zone de saisie du commentaire et le fil de discussion des commentaires en même temps. Pour économiser de l'espace vertical,
il masquera également tous les autres champs requis jusqu'à ce que le widget soit utilisé.

Cependant, le widget de commentaires peut être caché derrière un bouton, par exemple :

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='Widget de commentaires réduit derrière un bouton qui affiche le nombre de commentaires jusqu\'à ce qu\'un lecteur clique dessus'; title='Cliquez pour afficher les commentaires' app-screenshot-end]

Le bouton utilise un texte traduit différent selon que les commentaires sont actuellement affichés ou non. Si les commentaires sont masqués, il utilise `translations.SHOW_COMMENTS_BUTTON_TEXT`. Si les
commentaires sont affichés, il utilise `translations.HIDE_COMMENTS_BUTTON_TEXT`. Les traductions peuvent contenir le texte `[count]` qui sera remplacé par le nombre localisé.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = 'Cliquez pour afficher ou masquer les commentaires'; code-example-end]

Ceci est conçu pour remplacer la configuration `hideCommentsUnderCountTextFormat`.

Le compteur est mis à jour en temps réel avec le fil de commentaires. Le bouton n'est pas affiché s'il n'y a aucun commentaire.

Cela peut être activé sans code en créant une règle de personnalisation et en activant « Cliquez pour afficher les commentaires » :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='Case à cocher afficher les commentaires cochée dans une règle de personnalisation sur la page de personnalisation du widget'; title='Activer l\'affichage des commentaires' app-screenshot-end]