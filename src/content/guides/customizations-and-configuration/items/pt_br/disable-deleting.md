---
Por padrão, o FastComments permitirá que os usuários excluam seus comentários.

No entanto, é possível impedir isso.

Na página de personalização do widget, veja a opção "Desativar Exclusão".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='Opção Desativar Exclusão na página de personalização do widget, impedindo que os comentaristas removam seus comentários'; title='Desativar Exclusão de Comentário' app-screenshot-end]

- Isso afeta apenas os Comentadores regulares e não os moderadores ou administradores, que ainda poderão excluir.
- Isso também afetará integrações de API quando `contextUserId` for passado. 

---