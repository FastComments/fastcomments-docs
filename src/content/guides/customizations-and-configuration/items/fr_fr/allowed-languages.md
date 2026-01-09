Par défaut, FastComments ne limite pas les langues utilisées pour commenter. 

Il peut être souhaitable de limiter les langues qu'utilise une communauté.

Cela peut être configuré sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; title='Allowed Languages' app-screenshot-end]

Le système analysera leur commentaire et déterminera sa langue, puis la comparera à la liste autorisée.

Si le commentaire est rédigé dans une langue qui n'est pas autorisée, un message d'erreur localisé s'affiche.