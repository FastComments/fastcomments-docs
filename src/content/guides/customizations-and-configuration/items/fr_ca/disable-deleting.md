---
Par défaut, FastComments permettra aux utilisateurs de supprimer leurs commentaires.

Cependant, il est possible d'empêcher cela.

Dans la page de personnalisation du widget, consultez l'option "Disable Deleting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Cela n'affecte que les Commenters réguliers et pas les moderators ou les admins, qui pourront toujours supprimer.
- Cela affectera également les intégrations API lorsque `contextUserId` est passé. 

---