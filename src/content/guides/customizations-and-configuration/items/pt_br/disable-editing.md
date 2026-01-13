Por padrão, o FastComments permitirá que os usuários editem seus comentários.

No entanto, é possível impedir isso.

Na página de personalização do widget, veja a opção "Desativar edição".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- Isso afeta apenas comentaristas regulares e não moderadores ou administradores, que ainda poderão editar.
- Isso também afetará integrações da API quando `contextUserId` for passado.