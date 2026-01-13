Por padrão, os usuários podem excluir seus próprios comentários. Além disso, excluir o comentário deles exclui automaticamente
todos os comentários filhos e transitórios na thread. Esse comportamento também é em tempo real.

Você pode restringir isso das seguintes maneiras:

- Em vez disso, anonimize o comentário excluído (defina name e text como `[deleted]` ou um valor personalizado).
- Não permitir excluir comentários quando houver respostas. Uma mensagem de erro personalizável é exibida.
- Restringir a exclusão quando um comentário tiver respostas apenas para administradores e moderadores.

Isso pode ser configurado via a seção `Comment Thread Deletion` na interface de personalização do widget.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]