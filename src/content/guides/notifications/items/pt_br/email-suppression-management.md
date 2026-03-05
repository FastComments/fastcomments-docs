Quando e-mails enviados pelo FastComments retornam (bounce) ou são marcados como spam pelo destinatário, o provedor de e-mail adiciona esse endereço a uma
lista de supressão. O FastComments sincroniza essas listas de supressão diariamente para que novos e-mails não sejam enviados para endereços que
não podem recebê-los.

Usuários e moderadores com endereços de e-mail suprimidos não receberão nenhuma notificação por e-mail, incluindo notificações de resposta,
menção, alertas de administrador e e-mails de resumo. Um distintivo vermelho "E-mail Suprimido" aparecerá ao lado dos usuários
e moderadores afetados na interface de administração.

#### Visualizando E-mails Suprimidos

Administradores do tenant com permissão Gerenciar Dados podem visualizar e-mails suprimidos na
[página E-mails Suprimidos](https://fastcomments.com/auth/my-account/suppressed-emails), em Gerenciar Dados.

A página mostra uma tabela com todos os endereços de e-mail suprimidos associados aos usuários, moderadores e comentaristas do seu tenant.
Você pode filtrar por endereço de e-mail usando o campo de busca.

#### Removendo uma Supressão

Para remover uma supressão, clique no botão **Remover** ao lado da entrada na tabela. Você será levado a uma página de confirmação que mostrará os detalhes da supressão. Clique em **Confirmar remoção** para prosseguir.

Quando uma supressão é removida, o FastComments contata o provedor de e-mail para desbloquear o endereço e limpa a flag de supressão de todos os registros de usuário e moderador associados.

#### Limites de Taxa

Para prevenir abuso, as remoções são limitadas por taxa:

- Cada endereço de e-mail só pode ser dessuprimido uma vez a cada 30 dias.
- Cada tenant pode realizar até 5 remoções por mês calendário.

Se uma supressão reaparecer após a remoção, isso significa que o endereço de e-mail voltou a gerar bounce ou foi reportado como spam novamente. Nesse caso,
o problema subjacente de entregabilidade deve ser resolvido antes de tentar outra remoção.