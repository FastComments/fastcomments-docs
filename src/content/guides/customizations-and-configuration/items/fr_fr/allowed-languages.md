---
Par défaut, FastComments ne limite pas les langues utilisées pour commenter. 

Il peut être souhaitable de limiter les langues qu'une communauté utilise.

Cela peut être configuré sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allowed-languages']; selector = '.allowed-languages'; alt='Sélecteur de langues autorisées sur la page de personnalisation du widget pour limiter les langues que les commentaires peuvent utiliser'; title='Langues autorisées' app-screenshot-end]

Le système analysera leur commentaire, déterminera sa langue, puis la comparera à la liste autorisée.

Si le commentaire est rédigé dans une langue non autorisée, un message d'erreur localisé est affiché. 

---