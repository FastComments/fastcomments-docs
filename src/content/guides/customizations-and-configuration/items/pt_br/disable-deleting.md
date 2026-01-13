---
Por padrão, o FastComments permite que os usuários excluam seus comentários.

No entanto, é possível impedir isso.

Na página de personalização do widget, veja a opção "Desativar exclusão".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Isso afeta apenas Comentadores regulares e não moderadores ou administradores, que ainda poderão excluir.
- Isso também afetará integrações de API quando `contextUserId` for passado. 

---