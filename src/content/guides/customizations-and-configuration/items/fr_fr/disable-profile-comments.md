[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments affichera un onglet « Profile Comments » sur les profils des utilisateurs, permettant aux visiteurs de laisser des commentaires sur le profil de quelqu'un.

Cependant, nous pouvons désactiver cet onglet :

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Disable Profile Comments'; code-example-end]

Cela peut également être fait sans code. Sur la page de personnalisation du widget, voir la section « Disable Profile Comments ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; alt='Page de personnalisation du widget avec la case à cocher « Disable Profile Comments » cochée pour masquer l\'onglet des commentaires de profil'; title='Désactiver les commentaires de profil' app-screenshot-end]