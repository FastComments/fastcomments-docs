---
Par défaut, FastComments autorise les utilisateurs à modifier leurs commentaires.

Cependant, il est possible d'empêcher cela.

Dans la page de personnalisation du widget, consultez l'option « Désactiver l'édition ».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- Cela n'affecte que les commentateurs réguliers et pas les modérateurs ou les administrateurs, qui pourront toujours modifier.
- Cela affectera également les intégrations API lorsque `contextUserId` est transmis. 

---