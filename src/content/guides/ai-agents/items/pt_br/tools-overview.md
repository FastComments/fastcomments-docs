As **ferramentas** de um agente são as ações que ele pode executar. O formulário de edição do agente tem uma seção **Allowed tool calls** onde você marca as ferramentas que este agente pode usar, e uma seção **Approvals** onde você marca as ações que devem exigir aprovação humana antes de entrar em vigor.

Existem três níveis para qualquer ferramenta:

- **Não permitida** - o agente não pode vê-la nem usá-la.
- **Permitida, sem aprovação** - o agente a usa diretamente. Registrada no histórico de execuções.
- **Permitida, com aprovação** - a chamada do agente entra em fila para revisão humana e só é executada quando um humano aprova.

Ferramentas não permitidas são silenciosas: o agente não pode solicitá-las e a plataforma as recusa imediatamente. Ferramentas com aprovação obrigatória sempre passam pela [caixa de aprovações](#approval-workflow).

### Trilha de auditoria em cada ação

Cada ação executada pelo agente é registrada com uma justificativa curta (1-2 frases explicando o porquê) e uma pontuação de confiança (0.0-1.0). Ambos aparecem na [Visualização de Detalhes da Execução](#run-detail-view) e em cada [aprovação](#approval-workflow). Buscar na memória é a única exceção somente leitura: não é registrada como uma ação e está sempre disponível, independentemente da lista de permissões.

### Referência de ferramentas

#### Publicar comentários

Permite que o agente publique um comentário como ele mesmo. O comentário é exibido publicamente sob o nome de exibição do agente. Usado por agentes de saudação e resumo. Reversível - qualquer moderador pode remover um comentário ruim. Exija aprovação se sua comunidade precisar que toda mensagem pública seja revisada por humanos.

#### Editar um comentário

Permite que o agente reescreva o texto de um comentário dentro do escopo. O texto original é preservado no log de auditoria do comentário. Reserve para casos restritos - ocultar dados pessoais (PII) que um usuário vazou, ou corrigir uma resposta anterior do próprio agente. Não serve para reescrever opiniões ou suavizar o tom. Veja [Editar comentário](#tool-edit-comment) para a página completa.

#### Votar em comentários

Permite que o agente vote a favor ou contra um comentário. O voto conta para o total de votos do comentário como qualquer outro. A maioria das comunidades prefere não ter bots votando; não é habilitado em nenhum template inicial. Se você permitir, a votação é reversível.

#### Fixar / desafixar um comentário

Permite que o agente fixe um comentário no topo da página ou desafixe um que já esteja fixado. A plataforma não impõe uma regra de um fixe por thread, então um agente de fixação deve ser instruído a desafixar o comentário fixado anterior primeiro. Para descobrir o que já está fixado na mesma página, o agente pode chamar a ferramenta somente leitura `get_pinned_comments` (veja abaixo). Usada pelo template Top Comment Pinner.

#### Bloquear / desbloquear um comentário

Permite que o agente impeça novas respostas sob um comentário, ou restaure as respostas. O comentário bloqueado permanece visível. Útil para períodos de resfriamento em threads acaloradas, em conjunto com um desbloqueio adiado. Para descobrir o que está bloqueado atualmente na mesma página, o agente pode chamar a ferramenta somente leitura `get_locked_comments` (veja abaixo).

#### Marcar / desmarcar spam

Permite que o agente marque um comentário como spam (ocultando-o dos leitores e alimentando o classificador de spam) ou remova essa marcação. A ferramenta essencial para qualquer agente de moderação. Reversível.

#### Aprovar / desaprovar um comentário

Permite que o agente exiba aos leitores um comentário retido, ou oculte um que já esteja visível. Mais útil em tenants que retêm novos comentários para revisão de moderadores.

#### Marcar um comentário como revisado

Uma ferramenta de estado de fila: marca um comentário como "um moderador (ou agente) já olhou isto". Não altera a visibilidade. Baixo risco; raramente exige aprovação.

#### Conceder um badge

Permite que o agente dê a um usuário um badge que você configurou para seu tenant. Reversível por um moderador. Quando esta ferramenta está habilitada, o agente pode ver os badges do seu tenant e escolher o certo por conta própria, então você não precisa colar identificadores de badge nas diretrizes da comunidade ou no prompt inicial. Para direcionar qual badge é concedido para qual comportamento, referencie os badges pelo **Display Label** no prompt.

#### Enviar e-mail

Permite que o agente envie um e-mail em texto simples ao autor de um comentário no escopo do gatilho. O agente nunca vê o endereço de e-mail do destinatário - ele escolhe um comentário e a plataforma entrega ao endereço que aquele comentarista deixou ao postar. O endereço de origem é o remetente com a marca do seu tenant (com DKIM) quando o domínio do comentário corresponde a um domínio configurado, caso contrário o padrão da plataforma. Use com moderação - e-mail é a ferramenta de maior atrito e e-mails ruins são difíceis de desfazer.

#### Salvar / buscar memória do agente

Duas ferramentas pareadas que leem e escrevem um pool compartilhado de notas sobre o usuário para o qual um gatilho foi disparado. A memória é compartilhada entre todos os agentes do seu tenant, então as notas de um agente de triagem informam as decisões de um agente moderador. A busca é somente leitura e sempre disponível; salvar raramente exige aprovação. Veja [Sistema de Memória do Agente](#agent-memory-system) para o design completo.

#### Obter comentários fixados / Obter comentários bloqueados

Duas ferramentas de descoberta somente leitura que listam os comentários fixados (ou bloqueados) na mesma página (`urlId`) em que o gatilho foi disparado. Elas não recebem argumentos - a página é lida do contexto do gatilho, então o agente não pode mudar para outras páginas. Use-as quando um agente precisa agir sobre um comentário que já está fixado ou bloqueado - tipicamente a primeira chamada antes de `unpin_comment` ou `unlock_comment`, ou antes de fixar um novo comentário para que o existente possa ser desafixado primeiro.

Cada ferramenta é controlada separadamente em **Allowed tool calls** (o admin marca `List pinned comments on the current page` ou `List locked comments on the current page`). Elas não podem exigir aprovação - ferramentas somente leitura não têm efeito colateral a aprovar. Chamá-las não é registrado como uma ação no histórico de execuções; apenas a chamada resultante de `unpin_comment` / `unlock_comment` / `pin_comment` (se houver) aparece. A lista é limitada aos 20 resultados mais recentes por chamada.

Importante entender: quando uma dessas ferramentas retorna um commentId, esse commentId é adicionado ao escopo por execução do agente, então a chamada subsequente de `unpin_comment` / `unlock_comment` é validada contra a verificação de segurança de alvo de ferramenta da plataforma. Sem chamar primeiro a ferramenta de descoberta, o agente não pode agir sobre comentários que não estejam já no escopo imediato do gatilho. Então um agente do tipo desafixar normalmente tem ambas as ferramentas habilitadas (por exemplo, `get_pinned_comments` mais `unpin_comment`).

#### Advertir um usuário

Envia uma DM privada de advertência a um usuário sobre um comentário específico, e registra a advertência atomicamente na memória do agente. A política de escalonamento da plataforma é construída em torno desta ferramenta - advertir primeiro, banir apenas se o usuário reincidir. Veja [Advertir usuário](#tool-warn-user) para a página completa.

#### Banir um usuário

A ferramenta de maior consequência que um agente pode chamar. Bane um usuário com uma duração fixa, opcionalmente como shadow ban, opcionalmente também banindo o IP, opcionalmente também excluindo todos os comentários do usuário. As duas opções destrutivas (IP, excluir tudo) são controladas por opt-ins adicionais no formulário de edição. Na região da UE, todos os bans exigem aprovação humana (veja [Conformidade com o Artigo 17 do DSA da UE](#eu-dsa-compliance)). Veja [Banir usuário](#tool-ban-user) para a página completa.

### Subopções da ferramenta de banimento

A ferramenta de banimento expõe duas opções destrutivas - excluir todos os comentários e banir por IP - que ficam completamente ocultas do modelo até que você as habilite via a seção **Ban options** no formulário de edição. Mesmo que o modelo alucine o parâmetro, a plataforma recusa valores que você não habilitou. Veja [Banir usuário](#tool-ban-user).