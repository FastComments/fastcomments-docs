Na [página de Agentes de IA](https://fastcomments.com/auth/my-account/ai-agents) você pode criar um agente de duas maneiras:

- **A partir de um modelo.** Clique em **Browse templates** e escolha um dos quatro agentes iniciais incorporados. O formulário aparece pré-preenchido e o status do agente é **Dry Run**. Veja [Starter Templates](#starter-templates).
- **Do zero.** Clique em **Create new agent**. O formulário aparece em branco.

De qualquer forma, é o mesmo formulário de edição que você salva e edita depois. Esta página percorre o formulário de cima a baixo.

### Noções Básicas

- **Internal name.** Um identificador curto usado apenas nos painéis de administração (histórico de execuções, analytics, logs de auditoria). Minúsculas com underscores funcionam bem: `moderator`, `welcome_greeter`. Se o internal name de um modelo já estiver em uso, o formulário adiciona um sufixo automaticamente (`tos_enforcer_2`, etc.).
- **Display name.** Exibido publicamente sempre que o agente publica um comentário. É isso que seus leitores veem.
- **Status.** Disabled, Dry Run, or Enabled. Novos agentes sempre padrão para Dry Run. Veja [Status States](#status-states).

### Modelo

Escolha o LLM. Veja [Choosing a Model](#choosing-a-model).

### Orçamento

Limites opcionais diários e mensais na moeda da sua conta, além de uma checklist de **Alert thresholds** (padrão 80% e 100%). Veja [Budgets Overview](#budgets-overview) e [Budget Alerts](#budget-alerts).

### Personalidade

O **Initial prompt** é o system prompt que define tom, papel e regras de decisão. Texto simples, sem sintaxe de template. Veja [Personality and the Initial Prompt](#personality-prompt).

### Contexto

O fieldset Context contém três caixas de seleção, uma área de texto para diretrizes e os campos de escopo:

- Incluir o comentário pai e respostas anteriores no mesmo thread.
- Incluir o trust factor do comentarista, idade da conta, histórico de bans e comentários recentes.
- Incluir título da página, subtítulo, descrição e meta tags.
- Um bloco de texto opcional de **Community guidelines** que é pré-posto a cada prompt.
- **Restrict to specific pages** - allowlist de padrões de URL (um por linha). Vazio significa que vale para todo o tenant.
- **Restrict to specific locales** - allowlist de locales via um seletor de lista dupla. Vazio significa todos os locales.

Mais contexto produz decisões melhores, mas aumenta o custo em tokens por execução. Veja [Context Options](#context-options), [Community Guidelines](#community-guidelines) e [Scope: URL and Locale Filters](#scope-url-locale).

### Gatilhos

Escolha pelo menos um evento da lista. Para gatilhos vote-threshold e flag-threshold você também deve definir o threshold. O campo opcional **Delay before running** adia a execução após o disparo de um gatilho (útil para thresholds de flags onde os votos ainda estão se estabilizando). Veja [Trigger Events Overview](#triggers-overview) e [Deferred Triggers](#trigger-deferred-delay).

### Chamadas de ferramentas permitidas

Marque **Allow any tool calls** para expor a paleta completa de ferramentas. Caso contrário, marque as ferramentas específicas que o agente pode usar - ferramentas não permitidas são removidas da paleta do modelo e recusadas no momento do despacho. A subseção **Ban options** coloca as variantes destrutivas de ban (delete-all-comments, ban-by-IP) atrás de opt-ins explícitos. Veja [Allowed Tool Calls Overview](#tools-overview) e [Tool: ban_user](#tool-ban-user).

### Aprovações

Marque as ações que devem ser aprovadas por um humano antes que o agente as execute. Aprovações se aplicam somente às ferramentas que o agente tem permissão para invocar; ferramentas não permitidas são recusadas sumariamente. Na região da UE, **ban_user** é ativado por força do Artigo 17 do Digital Services Act. Veja [Approval Workflow](#approval-workflow) e [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Notificações de aprovação

Se as aprovações estiverem ativadas, escolha para quem é enviado email:

- **Todos os admins e moderadores** - account owners, super admins e comment moderator admins.
- **Usuários específicos** - selecionados manualmente via um seletor de lista dupla.

A frequência de entrega individual de cada revisor (imediata, digest horário, digest diário) é definida no próprio perfil. Veja [Approval Notifications](#approval-notifications).

### Estatísticas

Somente leitura. Total de execuções, timestamp da última execução e o ID do comentário mais recente escrito pelo agente (se houver).

### Salvar

Clique em **Save agent**. A página redireciona de volta para a lista de agentes. Novos agentes ficam imediatamente elegíveis para receber gatilhos em dry-run.

### Editar mais tarde

Cada linha na página de lista de agentes expõe ações por agente: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, e **Delete**. Editar um agente não retroage sobre execuções já registradas - o histórico é preservado. Snapshots de replay também congelam a configuração do agente no ponto em que o replay foi iniciado, então os resultados de um replay salvo permanecem reprodutíveis mesmo depois que você edita o prompt.