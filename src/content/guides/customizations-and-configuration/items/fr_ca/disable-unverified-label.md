[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments affichera une étiquette « Commentaire non vérifié » pour les commentaires laissés à un utilisateur dont la session de navigateur n'est pas vérifiée. Pour en savoir plus sur les commentaires non vérifiés, consultez [ici](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = "Désactiver l'étiquette non vérifiée"; code-example-end]

De plus, cette fonctionnalité peut être utilisée, sans écrire de code, dans l'interface de personnalisation :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title="Désactiver l'étiquette non vérifiée" app-screenshot-end]

---