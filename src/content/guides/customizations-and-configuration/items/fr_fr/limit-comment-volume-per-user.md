---
Par défaut, chaque utilisateur peut soumettre jusqu'à `5 comments` dans la même minute.

Cela est suivi par user id, anon user id, et ip address (hachée).

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Remarque : si vous utilisez la comment creation API, vous pouvez vouloir transmettre l'adresse `ip` originale de l'utilisateur dans la requête vers notre backend afin que le rate limiting soit appliqué
par utilisateur et non globalement à votre compte.
---