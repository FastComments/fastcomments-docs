---
Par défaut, FastComments autorise les utilisateurs à modifier leurs commentaires.

Cependant, il est possible d'empêcher cela.

Dans la page de personnalisation du widget, voyez l'option "Disable Editing".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; alt='Option Désactiver la modification sur la page de personnalisation du widget, empêchant les commentateurs de modifier leurs commentaires'; title='Désactiver la modification des commentaires' app-screenshot-end]

- Cela n'affecte que les commentateurs ordinaires et non les modérateurs ou administrateurs, qui pourront toujours modifier.
- Cela affectera également les intégrations API lorsque `contextUserId` est fourni. 

---