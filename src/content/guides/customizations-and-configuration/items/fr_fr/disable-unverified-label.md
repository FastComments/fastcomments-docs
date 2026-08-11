[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments affichera une étiquette « Commentaire non vérifié » pour les commentaires laissés pour un utilisateur qui a une session de navigateur non vérifiée. En savoir plus sur les commentaires non vérifiés [ici](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Désactiver l\'étiquette de commentaire non vérifié'; code-example-end]

De plus, cette fonctionnalité peut être utilisée, sans écrire de code, dans l'interface de personnalisation :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; alt='Page de personnalisation du widget avec la case à cocher Désactiver l\'étiquette de commentaire non vérifié cochée'; title='Désactiver l\'étiquette de commentaire non vérifié' app-screenshot-end]