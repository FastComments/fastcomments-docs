As **ferramentas** de um agente são as ações que ele pode executar. O formulário de edição do agente tem uma seção **Chamadas de ferramentas permitidas** onde você marca as ferramentas que este agente está autorizado a usar, e uma seção **Aprovações** onde você marca as ações que devem exigir aprovação humana antes de entrarem em vigor.

Existem três níveis para qualquer ferramenta:

- **Proibida** - o agente não pode vê-la nem usá-la.
- **Permitida, sem aprovação** - o agente a utiliza diretamente. Registrada no histórico de execução.
- **Permitida, com aprovação** - a chamada do agente é colocada em fila para revisão humana e só é executada quando um humano aprova.

Ferramentas proibidas são silenciosas: o agente não pode solicitá-las e a plataforma as recusa categoricamente. Ferramentas que exigem aprovação sempre passam pela [caixa de entrada de aprovações](#approval-workflow).

### Rastro de auditoria em cada ação

Toda ação realizada pelo agente é registrada com uma breve justificativa (1–2 frases explicando o motivo) e uma pontuação de confiança (0.0–1.0). Ambos aparecem na [Visualização de Detalhes da Execução](#run-detail-view) e em cada [aprovação](#approval-workflow). A busca na memória é a única exceção somente leitura: ela não é registrada como ação e está sempre disponível independentemente da lista de permissões.

### Referência de ferramentas

#### Publicar comentários

Permite que o agente publique um comentário como ele mesmo. O comentário é exibido publicamente sob o nome de exibição do agente. Usado por agentes de boas-vindas e de resumo. Reversível - qualquer moderador pode remover um comentário inadequado. Bloqueie atrás de aprovação se sua comunidade precisar que toda mensagem pública seja revisada por um humano.

#### Editar um comentário

Permite que o agente reescreva o texto de um comentário dentro do escopo. O texto original é preservado no registro de auditoria do comentário. Reserve para casos restritos - redigir PII que um usuário vazou, ou corrigir a própria resposta anterior do agente. Não para reescrever opiniões ou suavizar o tom. Veja [Editar comentário](#tool-edit-comment) para a página completa.

#### Votar em comentários

Permite que o agente vote a favor ou contra um comentário. O voto conta para o total de votos do comentário como qualquer outro. A maioria das comunidades prefere não ter bots votando; não habilitado em nenhum template inicial. Se você permitir, o voto é reversível.

#### Fixar / desfixar um comentário

Permite que o agente fixe um comentário no topo da página ou remova a fixação de um comentário que já está fixado. A plataforma não aplica uma regra de um único fixado por thread, então um agente que fixa comentários deve ser instruído a desfixar o comentário previamente fixado primeiro. Para descobrir o que já está fixado na mesma página, o agente pode chamar a ferramenta somente leitura `get_pinned_comments` (veja abaixo). Usado pelo template Top Comment Pinner.

#### Bloquear / desbloquear um comentário

Permite que o agente impeça novas respostas embaixo de um comentário, ou restaure as respostas. O comentário bloqueado continua visível. Útil para acalmar discussões aquecidas, combinado com um desbloqueio diferido. Para descobrir o que está atualmente bloqueado na mesma página, o agente pode chamar a ferramenta somente leitura `get_locked_comments`.

#### Marcar / desmarcar como spam

Permite que o agente marque um comentário como spam (ocultando-o dos leitores e alimentando o classificador de spam) ou remova essa marcação. A ferramenta básica para qualquer agente de moderação. Reversível.

#### Aprovar / desaprovar um comentário

Permite que o agente torne um comentário retido visível para os leitores, ou oculte um já visível. Mais útil em tenants que retenham novos comentários para revisão de moderadores.

#### Marcar um comentário como revisado

Uma ferramenta de estado de fila: marca um comentário como "um moderador (ou agente) analisou isto". Não altera a visibilidade. Baixo risco; raramente é sujeita a aprovação.

#### Conceder um badge

Permite que o agente dê a um usuário um badge que você configurou para seu tenant. Reversível por um moderador. Quando essa ferramenta está habilitada, o agente pode ver os badges do seu tenant e escolher o apropriado por conta própria, então você não precisa colar identificadores de badge nas diretrizes da comunidade ou no prompt inicial. Para orientar qual badge deve ser concedido por qual comportamento, refira-se aos badges pelo seu **Rótulo de exibição** no prompt.

#### Enviar email

Permite que o agente envie um email em texto simples para o autor de um comentário no escopo do gatilho. O agente nunca vê o endereço de email do destinatário - ele escolhe um comentário e a plataforma entrega para o endereço que o autor deixou ao postar. O endereço de remetente é o remetente com marca do seu tenant (com DKIM) quando o domínio do comentário corresponde a um domínio configurado, caso contrário é o padrão da plataforma. Use com parcimônia - email é a ferramenta de maior atrito e emails ruins são difíceis de desfazer.

#### Salvar / pesquisar memória do agente

Duas ferramentas emparelhadas que lêem e gravam um pool de notas compartilhado sobre o usuário para o qual um gatilho foi disparado. A memória é compartilhada entre todos os agentes do seu tenant, então as notas de um agente de triagem informam as decisões de um agente moderador. A pesquisa é somente leitura e está sempre disponível; salvar raramente é sujeito a aprovação. Veja [Sistema de Memória do Agente](#agent-memory-system) para o design completo.

#### Get pinned comments / Get locked comments

Duas ferramentas de descoberta somente leitura que listam os comentários fixados (ou bloqueados) na mesma página (`urlId`) em que o gatilho foi disparado. Elas não recebem argumentos - a página é lida a partir do contexto do gatilho, então o agente não pode mudar para outras páginas. Use-as quando um agente precisa agir sobre um comentário que já está fixado ou bloqueado - tipicamente a primeira chamada antes de `unpin_comment` ou `unlock_comment`, ou antes de fixar um novo comentário para que o existente possa ser desfixado primeiro.

Cada ferramenta é controlada separadamente em **Chamadas de ferramentas permitidas** (o administrador marca `List pinned comments on the current page` ou `List locked comments on the current page`). Elas não podem ser condicionadas à aprovação - ferramentas somente leitura não têm efeito colateral para aprovar. Chamá-las não é registrado como uma ação no histórico de execução; apenas a chamada resultante `unpin_comment` / `unlock_comment` / `pin_comment` (se houver) aparece. A lista é limitada às 20 correspondências mais recentes por chamada.

Importante entender: quando uma dessas ferramentas retorna um commentId, esse commentId é adicionado ao escopo por-execução do agente, então a chamada subsequente `unpin_comment` / `unlock_comment` valida contra a verificação de segurança de alvo da ferramenta da plataforma. Sem primeiro chamar a ferramenta de descoberta, o agente não pode agir em comentários que não estejam já no escopo imediato do gatilho. Assim, um agente do tipo desfixar normalmente tem ambas as ferramentas habilitadas (por exemplo `get_pinned_comments` mais `unpin_comment`).

#### Avisar um usuário

Envia uma DM privada alertando um usuário sobre um comentário específico, e registra atomicamente o aviso na memória do agente. A política de escalonamento da plataforma é construída em torno dessa ferramenta - avise primeiro, bloqueie apenas se o usuário reincidir. Veja [Avisar usuário](#tool-warn-user) para a página completa.

#### Banir um usuário

A ferramenta mais consequente que um agente pode chamar. Bloqueia um usuário por uma duração fixa, opcionalmente como shadow ban, opcionalmente também banindo o IP, opcionalmente também deletando todos os comentários do usuário. As duas opções destrutivas (IP, apagar-tudo) são controladas por opt-ins adicionais no formulário de edição. Na região da UE, todos os bans requerem aprovação humana (veja [Conformidade com o Artigo 17 do DSA da UE](#eu-dsa-compliance)). Veja [Banir usuário](#tool-ban-user) para a página completa.

### Sub-opções da ferramenta de banimento

A ferramenta Ban expõe duas opções destrutivas - delete-all-comments e ban-by-IP - que ficam ocultas para o modelo até que você as habilite via a seção **Opções de ban** no formulário de edição. Mesmo se o modelo alucinar o parâmetro, a plataforma recusa valores que você não optou por ativar. Veja [Banir usuário](#tool-ban-user).