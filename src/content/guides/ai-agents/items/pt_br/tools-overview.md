As ações que um agente pode executar são suas **ferramentas**. O formulário de edição do agente possui uma seção **Allowed tool calls** onde você marca as ferramentas que este agente tem permissão para usar, e uma seção **Approvals** onde você marca as ações que devem exigir aprovação humana antes de serem efetivadas.

Existem três níveis para qualquer ferramenta:

- **Disallowed** - o agente não pode vê-la nem usá-la.
- **Allowed, no approval** - o agente a usa diretamente. Registrado no histórico de execução.
- **Allowed, with approval** - a chamada do agente é enfileirada para revisão humana e só é executada quando um humano aprova.

Ferramentas Disallowed são silenciosas: o agente não pode solicitá-las e a plataforma as recusa imediatamente. Ferramentas protegidas por aprovação sempre passam pela [caixa de entrada de aprovações](#approval-workflow).

### Audit trail on every action

Toda ação que o agente realiza é registrada com uma breve justificativa (1–2 frases explicando o porquê) e uma pontuação de confiança (0.0–1.0). Ambos aparecem em [Run Detail View](#run-detail-view) e em cada [aprovação](#approval-workflow). A busca na memória é a única exceção somente leitura: ela não é registrada como ação e está sempre disponível independentemente da allowlist.

### Tool reference

#### Posting comments

Permite que o agente publique um comentário como ele mesmo. O comentário é exibido publicamente sob o nome de exibição do agente. Usado por agentes de boas-vindas e de sumarização. Reversível - qualquer moderador pode remover um comentário inadequado. Normalmente permitido sem aprovação; bloqueie-o por aprovação se sua comunidade precisar que toda mensagem pública seja revisada por um humano.

#### Editing a comment

Permite que o agente reescreva o texto de um comentário dentro do escopo. O texto original é preservado no log de auditoria do comentário. Reserve para casos restritos - redigir PII que um usuário divulgou, ou corrigir a própria resposta anterior do agente. Não para reescrever opiniões ou suavizar tom. **Considere fortemente exigir aprovação.** Veja [Editar comentário](#tool-edit-comment) para a página completa.

#### Voting on comments

Permite que o agente vote a favor ou contra um comentário. O voto conta para o total de votos do comentário como qualquer outro voto. A maioria das comunidades prefere que bots não votem; não está habilitado em nenhum template inicial. Se você permitir, o voto é reversível.

#### Pin / unpin a comment

Permite que o agente fixe um comentário no topo da página ou desfixe um comentário já fixado. A plataforma não impõe uma regra de um único fixado por tópico, então um agente responsável por fixar deve ser instruído a desfixar o comentário previamente fixado primeiro. Usado pelo template Top Comment Pinner. Reversível; geralmente permitido sem aprovação.

#### Lock / unlock a comment

Permite que o agente impeça novas respostas a um comentário ou restaure as respostas. O comentário bloqueado permanece visível. Útil para períodos de resfriamento em tópicos acalorados, combinado com um desbloqueio diferido. Reversível, mas visível para sua comunidade; considere exigir aprovação em comunidades de alto risco.

#### Mark / unmark spam

Permite que o agente marque um comentário como spam (ocultando-o dos leitores e alimentando o classificador de spam) ou remova essa marcação. A ferramenta fundamental para qualquer agente de moderação. Reversível. Considere fortemente exigir aprovação nas primeiras semanas enquanto você constrói confiança no agente.

#### Approve / un-approve a comment

Permite que o agente mostre um comentário retido aos leitores ou oculte um comentário já visível. Mais útil em tenants que retêm novos comentários para revisão por moderadores. Alto risco ao remover a aprovação de um comentário visível - considere exigir aprovação.

#### Mark a comment reviewed

Ferramenta de estado de fila: marca um comentário como "um moderador (ou agente) olhou isto". Não altera a visibilidade. Baixo risco; raramente exige aprovação.

#### Award a badge

Permite que o agente dê a um usuário um distintivo que você configurou para seu tenant. Reversível por um moderador. Raramente requer aprovação. Quando esta ferramenta está habilitada, o agente pode ver os distintivos do seu tenant e escolher o apropriado por conta própria, então você não precisa colar identificadores de distintivos em suas diretrizes da comunidade ou no prompt inicial. Se quiser orientar qual distintivo deve ser concedido para determinado comportamento, refira-se aos distintivos pelo seu **Rótulo de Exibição** no prompt.

#### Send email

Permite que o agente envie um e-mail em texto simples ao autor de um comentário dentro do escopo do gatilho. O agente nunca vê o endereço de e-mail do destinatário - ele escolhe um comentário e a plataforma entrega para o endereço que o comentarista deixou ao postar. O from-address é o remetente com a marca do seu tenant (com DKIM) quando o domínio do comentário corresponde a um domínio configurado; caso contrário, é o padrão da plataforma. Use com parcimônia - o e-mail é a ferramenta com maior atrito e e-mails ruins são difíceis de desfazer. Considere fortemente exigir aprovação, e encaminhe os e-mails de aprovação para quem for responsável pela caixa de entrada que o agente acabará usando.

#### Save / search agent memory

Duas ferramentas pareadas que leem e escrevem um pool compartilhado de notas sobre o usuário para o qual um gatilho foi acionado. A memória é compartilhada entre todos os agentes do seu tenant, então as notas de um agente de triagem informam as decisões de um agente moderador. A busca é somente leitura e sempre disponível; salvar raramente exige aprovação. Veja [Sistema de Memória do Agente](#agent-memory-system) para o desenho completo.

#### Warn a user

Envia uma mensagem privada (DM) avisando um usuário sobre um comentário específico, e registra atomically o aviso na memória do agente. A política de escalonamento da plataforma é construída em torno desta ferramenta - avise primeiro, ban só se o usuário reincidir. Menos comumente exigida aprovação do que `ban_user`, mas considere exigir aprovação durante as primeiras semanas de vida de um agente. Veja [Avisar usuário](#tool-warn-user) para a página completa.

#### Ban a user

A ferramenta mais consequente que um agente pode chamar. Banimento de um usuário por uma duração fixa, opcionalmente como shadow ban, opcionalmente também banindo o IP, opcionalmente também deletando todos os comentários do usuário. As duas opções destrutivas (IP, delete-all) são protegidas por opt-ins extras no formulário de edição. Na região da UE, todos os banimentos requerem aprovação humana (veja [Conformidade com o Artigo 17 da DSA da UE](#eu-dsa-compliance)). Considere fortemente exigir aprovação em todos os lugares. Veja [Banir usuário](#tool-ban-user) para a página completa.

### Ban-tool sub-options

A ferramenta de Ban expõe duas opções destrutivas - delete-all-comments e ban-by-IP - que ficam ocultas ao modelo completamente até que você as ative via a seção **Opções de banimento** no formulário de edição. Mesmo que o modelo alucine o parâmetro, a plataforma recusa valores que você não tenha optado por aceitar. Veja [Banir usuário](#tool-ban-user).