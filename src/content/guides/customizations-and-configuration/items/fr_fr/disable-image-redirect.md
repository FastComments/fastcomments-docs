[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments permet aux utilisateurs de télécharger des images. Lorsqu'un utilisateur clique sur cette image, FastComments ouvrira, par défaut,
un nouvel onglet pour afficher l'image en entier. Définir ce drapeau sur true désactive ce comportement :

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Si vous ne prévoyez pas de gérer vous-même le clic sur l'image (voir [onImageClicked](#callbacks)), nous recommandons de combiner cela avec un style
pour supprimer l'aspect cliquable de l'image.