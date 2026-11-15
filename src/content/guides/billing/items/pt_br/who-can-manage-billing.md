O faturamento é restrito a dois tipos de usuários:

- O `Account Owner`, o usuário que criou a conta.
- Qualquer usuário com a permissão `Billing Admin`.

Eles são os únicos usuários que podem alterar o plano, salvar ou substituir um cartão, abrir as `Invoice Settings`, definir limites de gastos do Flex e adicionar crédito. Qualquer outra pessoa que tentar verá `Only the account owner or a billing admin can change the plan or payment method.`

E-mails de faturamento, como o lembrete no meio do período de teste, avisos de pagamento falho e notificações de desativação, são enviados aos usuários com a permissão `Billing Admin`. Conceda a permissão a quem deve recebê-los, por exemplo, alguém do setor financeiro, e essa pessoa receberá os e-mails sem precisar de qualquer outro acesso ao painel. Os lembretes enviados após o término de um período de teste sem informações de faturamento vão para todos os usuários do painel da conta. Cada um desses e-mails tem um link de cancelamento de inscrição; um usuário que optar por sair das notificações da conta para de recebê-los.

As faturas em si vão para o `Billing Email` nas [Invoice Settings](#invoice-settings), que não precisa pertencer a um usuário do painel.

A página [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) requer a permissão `Analytics Admin` em vez disso, pois ela mostra o uso em vez dos detalhes de pagamento.

Para conceder ou remover a permissão, abra a página `Users` e edite o usuário. Veja [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). No Flex, lembre-se de que cada administrador adicional é um assento cobrado, veja [Admin and Moderator Seats](#admin-and-moderator-seats).

Se você estiver conectado e não conseguir ver `Billing Info` de forma alguma, ou se for redirecionado para a página inicial, você está conectado com uma conta de comentarista em vez de uma conta de usuário do painel. Saia e entre com a conta que possui o site.