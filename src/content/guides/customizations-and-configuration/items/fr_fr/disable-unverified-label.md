---
[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments affichera un label "Commentaire non vérifié" pour les commentaires qui ont été laissés pour un utilisateur qui
a une session de navigateur non vérifiée. En savoir plus sur les commentaires non vérifiés [ici](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

De plus, cette fonctionnalité peut être utilisée, sans écrire de code, dans l'interface de personnalisation :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---