---
Por padrão, o FastComments permitirá que os usuários editem seus comentários.

No entanto, é possível impedir isso.

Na página de personalização do widget, veja a opção "Desativar edição".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; alt='Opção Desativar Edição na página de personalização do widget, impedindo que os comentaristas editem seus comentários'; title='Desativar Edição de Comentário' app-screenshot-end]

- Isso afeta apenas os Comentadores regulares e não os moderadores ou administradores, que ainda poderão editar.
- Isso também afetará integrações de API quando `contextUserId` for passado. 

---