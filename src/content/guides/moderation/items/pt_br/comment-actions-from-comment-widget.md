Um subconjunto das ações de moderação pode ser feito diretamente no thread de comentários, sem a necessidade de ir para a página de Moderação de Comentários.

Quando você estiver logado, clique no botão de editar no canto superior direito de um comentário. Como moderador, você deve ter as seguintes opções:

- **Fixar** esse comentário
- **Excluir** esse comentário
- **Excluir** esse comentário + **Banir o usuário** (Permanente ou Shadow, mais detalhes abaixo)
- **Editar** esse comentário
- **Bloquear** ou **Desbloquear** esse comentário (mais detalhes abaixo)
- Marcar esse comentário como **Aprovado** (mostrá-lo) ou **Não Aprovado** (ocultá-lo)
- Marcar esse comentário como **Spam** ou **Não Spam**

### Bloqueando um Comentário

Bloquear um comentário individual impede quaisquer novas respostas a ele, e também impede que o próprio comentário seja editado ou excluído até que seja desbloqueado. Isso se aplica a todos, incluindo administradores e moderadores. Se você precisar editar ou remover um comentário bloqueado, desbloqueie-o primeiro, faça a alteração e bloqueie-o novamente se desejar.

Um ícone de cadeado aparece no canto superior direito de um comentário bloqueado para que os leitores possam ver, de relance, que o thread está fechado. As entradas de menu Editar e Excluir são ocultas para comentários bloqueados tanto no widget de comentários quanto na API pública (`PATCH` e `DELETE` retornam `code: 'locked'` se chamados contra um comentário bloqueado).

Duas exceções intencionais contornam o bloqueio, porque, caso contrário, deixariam dados órfãos: quando um usuário exclui toda a sua conta (seus comentários são limpos independentemente do estado do bloqueio), e quando um moderador bane um usuário com a opção "delete all comments from this user" (a varredura remove os bloqueios).

### Fechando Threads de Comentários

Moderadores e administradores podem bloquear, ou fechar, threads de comentários, selecionando `Close Thread` no menu de três pontos no topo da área de comentários, se estiverem logados. Eles podem selecionar `Re-Open Thread` mais tarde, a qualquer momento, para reabrir os comentários.

Fechar um thread de comentários impede novos comentários, mas ainda permite votação e que os usuários excluam seus comentários, se desejarem.

Fechar e reabrir threads de comentários afeta instantaneamente todos os usuários que estão visualizando o thread.

Você também pode marcar um thread como somente leitura, o que remove as opções de voto e exclusão também, criando uma regra de personalização especificamente para essa página.

### Atualizado em Tempo Real

Todas essas ações irão atualizar os threads de comentários de outros usuários imediatamente, sem que eles precisem recarregar a página. Entretanto, ações de moderador como ocultar um comentário ou marcá-lo como spam não removem o comentário da **tela do moderador** para que, se necessário, ele possa desfazer a ação rapidamente. Para indicar que o comentário está oculto, ele será destacado em comparação com os outros comentários (a cor do destaque dependendo do motivo da remoção).

Por exemplo, dados os usuários `A (commenter)`, `B (Moderator 1)`, e `C (Moderator 2)`.

...e o seguinte cenário:

1. `User B (Moderator 1)` oculta um comentário.
2. Para `User A (commenter)` esse comentário é imediatamente ocultado.
3. Para `User C (Moderator 2)` esse comentário é imediatamente ocultado.
4. Para o usuário que fez a alteração, `User B (Moderator 1)`, o comentário permanece na tela dele, mas é destacado como removido. Ele tem a opção de desfazer sua ação, caso em que os outros usuários verão a atualização, ao vivo, novamente.