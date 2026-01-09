---
Par défaut, FastComments permet aux utilisateurs de supprimer leurs commentaires.

Cependant, il est possible d'empêcher cela.

Sur la page de personnalisation du widget, consultez l'option « Désactiver la suppression ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Cela n'affecte que les commentateurs réguliers et pas les modérateurs ou administrateurs, qui pourront toujours supprimer des commentaires.
- Cela affectera également les intégrations API lorsque `contextUserId` est passé. 

---