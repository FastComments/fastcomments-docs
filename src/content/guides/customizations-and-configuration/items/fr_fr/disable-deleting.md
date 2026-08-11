---
Par défaut, FastComments autorise les utilisateurs à supprimer leurs commentaires.

Cependant, il est possible d'empêcher cela.

Dans la page de personnalisation du widget, voyez l'option "Disable Deleting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='Option Désactiver la suppression sur la page de personnalisation du widget, empêchant les commentateurs de supprimer leurs commentaires'; title='Désactiver la suppression des commentaires' app-screenshot-end]

- Cela n'affecte que les commentateurs ordinaires et non les modérateurs ou administrateurs, qui pourront toujours supprimer.  
- Cela affectera également les intégrations API lorsque `contextUserId` est passé. 

---