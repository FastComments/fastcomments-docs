As **ferramentas** de um agente são as ações que ele pode executar. O formulário de edição do agente tem uma seção **Chamadas de ferramenta permitidas** onde você marca as ferramentas que este agente tem permissão para usar, e uma seção **Aprovações** onde você marca as ações que devem exigir a aprovação de um humano antes de entrarem em vigor.

Existem três níveis para qualquer ferramenta:

- **Negado** - o agente não pode ver ou usar a ferramenta.
- **Permitido, sem aprovação** - o agente a usa diretamente. Registrado no histórico de execuções.
- **Permitido, com aprovação** - a chamada do agente é enfileirada para revisão humana e só é executada quando um humano aprova.

Ferramentas negadas são silenciosas: o agente não pode solicitá-las e a plataforma as recusa imediatamente. Ferramentas que exigem aprovação sempre passam pela [caixa de entrada de aprovações](#approval-workflow).

### Registro de auditoria para cada ação

Cada ação que o agente realiza é registrada com uma breve justificativa (1–2 frases explicando o porquê) e uma pontuação de confiança (0.0–1.0). Ambos aparecem na [Visualização de Detalhe da Execução](#run-detail-view) e em cada [aprovação](#approval-workflow). A busca na memória é a única exceção somente-leitura: ela não é registrada como uma ação e está sempre disponível independentemente da lista de permissões.

### Referência das ferramentas

#### Publicar comentários

Permite que o agente publique um comentário como ele mesmo. O comentário é exibido publicamente sob o nome de exibição do agente. Usado por agentes de boas-vindas e de sumarização. Reversível — qualquer moderador pode remover um comentário impróprio. Geralmente permitido sem aprovação; exija aprovação se sua comunidade precisar que cada mensagem pública seja revisada por um humano.

#### Votar em comentários

Permite que o agente vote a favor ou contra um comentário. O voto conta para o total de votos do comentário como qualquer outro voto. A maioria das comunidades prefere que bots não votem; não habilitado em nenhum template inicial. Se você permitir, o voto é reversível.

#### Fixar / desafixar um comentário

Permite que o agente fixe um comentário no topo da página ou desafixe um que já esteja fixado. A plataforma não aplica uma regra de um fixo por tópico, então um agente encarregado de fixar deve ser instruído a desafixar o comentário fixado anterior primeiro. Usado pelo modelo Top Comment Pinner. Reversível; geralmente permitido sem aprovação.

#### Bloquear / desbloquear um comentário

Permite que o agente impeça novas respostas a um comentário, ou restaure as respostas. O comentário bloqueado permanece visível. Útil para períodos de resfriamento em discussões acaloradas, combinado com um desbloqueio adiado. Reversível, mas visível para sua comunidade; considere exigir aprovação em comunidades de alto risco.

#### Marcar / desmarcar spam

Permite que o agente marque um comentário como spam (ocultando-o dos leitores e alimentando o classificador de spam) ou limpe essa marcação. A ferramenta essencial para qualquer agente de moderação. Reversível. Considere fortemente exigir aprovação nas primeiras semanas enquanto você constrói confiança no agente.

#### Aprovar / desaprovar um comentário

Permite que o agente mostre um comentário retido para os leitores, ou oculte um já visível. Mais útil em tenants que retêm novos comentários para revisão de moderadores. Tem alto risco ao desaprovar um comentário visível — considere exigir aprovação.

#### Marcar um comentário como revisado

Uma ferramenta de estado de fila: marca um comentário como "um moderador (ou agente) viu isto." Não altera a visibilidade. Baixo risco; raramente exige aprovação.

#### Conceder um distintivo

Permite que o agente dê a um usuário um distintivo a partir da configuração de distintivos do seu tenant. Reversível por um moderador. Raramente exige aprovação. O agente deve conhecer o ID do distintivo, então inclua os IDs relevantes nas suas [diretrizes da comunidade](#community-guidelines) ou no seu [prompt inicial](#personality-prompt).

#### Enviar e-mail

Permite que o agente envie um e‑mail em texto simples de `noreply@fastcomments.com` para um endereço que ele escolher. Use com parcimônia — e‑mail é a ferramenta de maior atrito e e‑mails equivocados são difíceis de desfazer. Considere fortemente exigir aprovação, e encaminhe os e‑mails de aprovação para quem for dono da caixa de entrada para a qual o agente acabará enviando mensagens.

#### Salvar / buscar memória do agente

Duas ferramentas emparelhadas que leem e escrevem um pool compartilhado de notas sobre o usuário para o qual um gatilho foi acionado. A memória é compartilhada entre todos os agentes do seu tenant, então as notas de um agente de triagem informam as decisões de um agente moderador. A busca é somente leitura e está sempre disponível; salvar raramente exige aprovação. Veja o [Sistema de Memória do Agente](#agent-memory-system) para o design completo.

#### Avisar um usuário

Envia uma mensagem privada (DM) de advertência a um usuário sobre um comentário específico, e registra atomica‑mente a advertência na memória do agente. A política de escalonamento da plataforma é construída em torno desta ferramenta — avise primeiro, bane apenas se o usuário reincidir. Menos comumente bloqueada que `ban_user`, mas considere exigir aprovação durante as primeiras semanas de vida de um agente. Veja [Avisar usuário](#tool-warn-user) para a página completa.

#### Banir um usuário

A ferramenta mais consequente que um agente pode acionar. Bana um usuário por uma duração fixa, opcionalmente como um shadow ban, opcionalmente também banindo o IP, opcionalmente também excluindo todos os comentários do usuário. As duas opções destrutivas (IP, delete-all) estão protegidas por opt‑ins adicionais no formulário de edição. Mesmo que o modelo alucine o parâmetro, a plataforma recusa valores nos quais você não optou. Veja [Banir usuário](#tool-ban-user) para a página completa.

### Sub-opções da ferramenta de ban

A ferramenta Ban expõe duas opções destrutivas — delete-all-comments e ban-by-IP — que ficam completamente escondidas do modelo até que você opte por elas via a seção **Ban options** no formulário de edição. Mesmo que o modelo alucine o parâmetro, a plataforma recusa valores nos quais você não optou. Veja [Banir usuário](#tool-ban-user).