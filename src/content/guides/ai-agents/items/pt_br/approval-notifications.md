Quando o agente enfileira uma aprovação, a plataforma notifica os revisores por e-mail. Duas configurações no formulário de edição controlam isso: **quem** é notificado e **com que frequência**.

### Quem: modo de notificação

Dois modos:

- **Todos os admins e moderadores** (padrão) - todo proprietário de conta, super admin e administrador moderador de comentários no tenant é um revisor candidato.
- **Usuários específicos** - selecione manualmente uma lista usando um seletor de dupla lista no formulário de edição.

De qualquer forma, um revisor candidato deve ter uma conta no tenant e um endereço de e-mail válido para receber notificações.

### Com que frequência: frequência por usuário

A **própria conta** de cada revisor candidato define sua frequência pessoal de notificação para aprovações de agentes:

- **Imediato** (padrão) - um e-mail por aprovação pendente, enviado assim que a aprovação é criada.
- **Horária** - um e-mail resumo por hora com todas as aprovações enfileiradas naquela hora.
- **Diária** - um e-mail resumo a cada 24 horas.
- **Desativado** - nenhum e-mail. O usuário ainda pode revisar aprovações pela UI da caixa de entrada; ele apenas não é notificado.

O usuário altera essa configuração em seu próprio perfil, não no formulário de edição do agente. Isso é intencional - um tenant pode ter dez agentes, e um moderador não deveria ter que definir sua frequência preferida em cada agente individualmente.

### Jobs cron que geram os resumos

- **`hourly-agent-approval-digest`** - varre a cada hora, agrupa aprovações enfileiradas desde o último resumo de cada usuário, envia um e-mail por usuário.
- **`daily-agent-approval-digest`** - o mesmo, diariamente.
- **`agent-approval-reaper`** - remove aprovações que ultrapassaram 90 dias independentemente do estado.

Os cron jobs de resumo horária e diária são delimitados por destinatário: um usuário com frequência horária é processado pelo cron horário e ignorado pelo diário (e vice-versa). Usuários com frequência imediata são notificados pelo caminho de criação de aprovação, não pelos crons.

### Estado de deduplicação

A plataforma rastreia quais usuários já receberam e-mail sobre cada aprovação. Uma vez que um usuário foi notificado (imediatamente ou em um resumo), ele não receberá outro e-mail sobre a mesma aprovação - mesmo se mudar sua frequência de imediato para diária no meio do ciclo.

### Aprovação pelo e-mail

Cada e-mail de notificação contém um link de login assinado de um clique que leva o revisor diretamente para a página de detalhes da aprovação, já autenticado. Eles podem aprovar, rejeitar ou abrir o fluxo [Refine Prompts](#refining-prompts) a partir daí.

### E se não existirem admins

Se `notifyMode` for `All admins and moderators` mas o tenant não tiver super admins, administradores moderadores de comentários ou proprietários de conta com e-mails válidos, a plataforma registra um aviso e a aprovação ainda é enfileirada - apenas ninguém é notificado sobre ela. Ela ficará na caixa de entrada até que alguém a veja por acaso.

Se `notifyMode` for `Specific users` mas você não selecionou nenhum usuário, o resultado é o mesmo.

### E se as notificações de cobrança estiverem desativadas

[Budget Alerts](#budget-alerts) - os e-mails relacionados ao orçamento - vão para os admins de cobrança **independentemente da preferência de notificação por usuário**. Isso é intencional: estouros de orçamento afetam custo, e o responsável pela cobrança precisa saber.

As notificações de aprovação obedecem apenas à configuração de frequência agent-approval por usuário. Elas não verificam a opção de opt-out de notificações administrativas mais ampla - um usuário que optou por não receber notificações administrativas ainda receberá e-mails de aprovação se estiver na lista de revisores, a menos que sua frequência de agent-approval esteja definida como **Desativado**.

### Veja também

- [Approval Workflow](#approval-workflow) para o ciclo de vida completo de uma aprovação.
- [Refining Prompts](#refining-prompts) para o fluxo "Eu continuo aprovando o mesmo tipo de erro".