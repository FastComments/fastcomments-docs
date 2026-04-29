As **ferramentas** de um agente são as ações que ele pode tomar. O formulário de edição do agente tem uma seção **Chamadas de ferramenta permitidas** onde você marca as ferramentas que este agente tem permissão para usar, e uma seção **Aprovações** onde você marca as ações que devem exigir a aprovação de um humano antes de entrarem em vigor.

There are three levels for any tool:

- **Disallowed** - the agent cannot see or use it.
- **Allowed, no approval** - the agent uses it directly. Recorded in run history.
- **Allowed, with approval** - the agent's call is queued for human review and only runs when a human approves.

As ferramentas não permitidas são silenciosas: o agente não pode solicitá-las e a plataforma as recusa categoricamente. Ferramentas que exigem aprovação sempre passam pela [caixa de entrada de aprovações](#approval-workflow).

### Trilha de auditoria em cada ação

Every action the agent takes is recorded with a short justification (1-2 sentences explaining why) and a confidence score (0.0-1.0). Both appear in [Visualização de Detalhes da Execução](#run-detail-view) and on every [aprovação](#approval-workflow). Searching memory is the one read-only exception: it is not recorded as an action and is always available regardless of the allowlist.

### Referência de ferramentas

#### Postar comentários

Permite que o agente publique um comentário como ele mesmo. O comentário é exibido publicamente sob o nome de exibição do agente. Usado por agentes cumprimentadores e sumarizadores. Reversível - qualquer moderador pode remover um comentário ruim. Normalmente permitido sem aprovação; coloque sob aprovação se sua comunidade precisar que toda mensagem pública seja revisada por um humano.

#### Editar um comentário

Permite que o agente reescreva o texto de um comentário dentro do escopo. O texto original é preservado no log de auditoria do comentário. Reserve para casos restritos - remover PII que um usuário vazou, ou emendar a própria resposta anterior do agente. Não para reescrever opiniões ou suavizar o tom. **Considere fortemente colocar sob aprovação.** Veja [Editar comentário](#tool-edit-comment) para a página completa.

#### Votar em comentários

Permite que o agente vote a favor ou contra um comentário. O voto conta para o total de votos do comentário como qualquer outro voto. A maioria das comunidades prefere não ter bots votando; não habilitado em nenhum template inicial. Se você permitir, o voto é reversível.

#### Fixar / desafixar um comentário

Permite que o agente fixe um comentário no topo da página ou remova a fixação de um que já esteja fixado. A plataforma não aplica uma regra de um pin por tópico, portanto um agente que fixa comentários deve ser instruído a desafixar o comentário previamente fixado primeiro. Utilizado pelo template Top Comment Pinner. Reversível; normalmente permitido sem aprovação.

#### Bloquear / desbloquear um comentário

Permite que o agente impeça novas respostas sob um comentário, ou restaure as respostas. O comentário bloqueado continua visível. Útil para períodos de resfriamento em tópicos acalorados, combinado com um desbloqueio diferido. Reversível, mas visível para sua comunidade; considere colocar sob aprovação em comunidades de alto risco.

#### Marcar / desmarcar como spam

Permite que o agente marque um comentário como spam (ocultando-o dos leitores e alimentando o classificador de spam) ou limpe essa marcação. A ferramenta básica para qualquer agente de moderação. Reversível. Considere fortemente colocar sob aprovação nas primeiras semanas enquanto você constrói confiança no agente.

#### Aprovar / desaprovar um comentário

Permite que o agente mostre um comentário retido aos leitores, ou oculte um já visível. Mais útil em tenants que retêm novos comentários para revisão por moderadores. Alto risco ao desaprovar um comentário visível - considere colocar sob aprovação.

#### Marcar um comentário como revisado

Uma ferramenta de estado de fila: marca um comentário como "um moderador (ou agente) olhou isto." Não altera a visibilidade. Baixo risco; raramente colocada sob aprovação.

#### Conceder um distintivo

Permite que o agente dê a um usuário um distintivo a partir da configuração de distintivos do seu tenant. Reversível por um moderador. Raramente colocado sob aprovação. O agente deve saber o badge ID, então inclua os IDs relevantes nas suas [diretrizes da comunidade](#community-guidelines) ou no [prompt inicial](#personality-prompt).

#### Enviar email

Permite que o agente envie um email em texto puro de `noreply@fastcomments.com` para um endereço que ele escolher. Use com parcimônia - email é a ferramenta de maior atrito e emails problemáticos são difíceis de desfazer. Considere fortemente colocá-la sob aprovação, e encaminhe os emails de aprovação para quem for responsável pela caixa de entrada para a qual o agente acabará enviando mensagens.

#### Salvar / buscar memória do agente

Duas ferramentas pareadas que leem e escrevem um pool compartilhado de notas sobre o usuário para quem um gatilho foi acionado. A memória é compartilhada entre todos os agentes do seu tenant, então as notas de um agente de triagem informam as decisões de um agente moderador. A busca é somente leitura e está sempre disponível; salvar raramente é colocado sob aprovação. Veja [Sistema de Memória do Agente](#agent-memory-system) para o design completo.

#### Avisar um usuário

Envia uma DM privada avisando um usuário sobre um comentário específico, e registra atomically o aviso na memória do agente. A política de escalonamento da plataforma é construída em torno desta ferramenta - avise primeiro, bane apenas se o usuário reincidir. Menos comumente colocada sob aprovação do que `ban_user`, mas considere colocá-la sob aprovação nas primeiras semanas de vida de um agente. Veja [Avisar usuário](#tool-warn-user) para a página completa.

#### Banir um usuário

A ferramenta mais consequente que um agente pode chamar. Bana um usuário por uma duração fixa, opcionalmente como um shadow ban, opcionalmente também banindo o IP, opcionalmente também apagando todos os comentários do usuário. As duas opções destrutivas (IP, delete-all) estão protegidas por escolhas adicionais no formulário de edição. Mesmo que o modelo fabrique o parâmetro, a plataforma recusa valores nos quais você não optou. Veja [Banir usuário](#tool-ban-user) para a página completa.

### Subopções da ferramenta de banimento

A ferramenta de Ban expõe duas opções destrutivas - delete-all-comments e ban-by-IP - que ficam ocultas ao modelo inteiramente até você ativá-las via a seção **Opções de banimento** no formulário de edição. Mesmo se o modelo alucinar o parâmetro, a plataforma recusa valores nos quais você não optou. Veja [Banir usuário](#tool-ban-user).