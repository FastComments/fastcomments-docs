---
[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Le fait de définir `noNewRootComments` sur `true` fera en sorte que le widget masque la zone de réponse racine, mais permettra toujours aux utilisateurs de répondre
aux commentaires enfants. Par exemple, vous pourriez définir cela de façon conditionnelle au chargement de la page pour n'autoriser que certains utilisateurs à laisser des commentaires de premier niveau.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]

---