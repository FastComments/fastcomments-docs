---
Por padrão, os usuários podem excluir seus próprios comentários. Além disso, excluir seu comentário exclui automaticamente todos os comentários filhos e transitórios na thread. Esse comportamento também está ativo.

Você pode restringir isso das seguintes maneiras:

- Em vez disso, anonimizar o comentário excluído (definir nome e texto como `[deleted]` ou um valor personalizado).
- Não permitir a exclusão de comentários quando houver respostas. Uma mensagem de erro personalizável é exibida.
- Restringir a exclusão de comentários que têm respostas apenas a administradores e moderadores.

Isso pode ser configurado na seção `Comment Thread Deletion` na UI de Personalização do Widget.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; alt='Opções de exclusão de thread de comentários na UI de personalização do widget para anonimizar ou restringir exclusões com respostas'; title='Personalizar comportamento de exclusão para respostas' app-screenshot-end]

---