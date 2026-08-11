Par défaut, chaque utilisateur peut soumettre jusqu'à `5 comments` dans la même minute.

Ceci est suivi par l'ID utilisateur, l'ID utilisateur anonyme et l'adresse IP (hachée).

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Champ Max commentaires par minute sur la page de personnalisation du widget, réglé à 5 par défaut'; title='Limitation du volume de commentaires par utilisateur' app-screenshot-end]

Notez que si vous utilisez l'API de création de commentaires, vous pouvez vouloir transmettre l'adresse `ip` originale de l'utilisateur dans la requête à notre backend afin que la limitation de débit soit appliquée
par utilisateur et non globalement à votre compte.