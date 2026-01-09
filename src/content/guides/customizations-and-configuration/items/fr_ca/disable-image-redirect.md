[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments permet aux utilisateurs de téléverser des images. Lorsqu'un utilisateur clique sur cette image, FastComments ouvrira, par défaut,
un nouvel onglet pour afficher l'image en entier. Définir ce drapeau sur true désactive ce comportement :

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Si vous ne prévoyez pas d'intercepter le clic sur l'image vous-même (voir [onImageClicked](#callbacks)), nous recommandons de combiner cela avec du styling
pour supprimer l'apparence que l'image peut être cliquée.