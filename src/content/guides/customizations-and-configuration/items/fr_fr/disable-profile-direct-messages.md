[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

Par défaut, FastComments affichera un onglet "Direct Messages" sur les profils des utilisateurs, permettant aux visiteurs d'envoyer des messages directs à un utilisateur.

Cependant, nous pouvons désactiver cet onglet :

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Désactiver les messages directs du profil'; code-example-end]

Cela peut également être fait sans code. Dans la page de personnalisation du widget, voir la section « Désactiver les messages directs ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='Page de personnalisation du widget avec la case à cocher Désactiver les messages directs cochée pour masquer l\'onglet des messages du profil'; title='Désactiver les messages directs du profil' app-screenshot-end]

---