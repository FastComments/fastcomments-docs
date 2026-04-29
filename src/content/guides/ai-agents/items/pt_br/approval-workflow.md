Uma **aprovação** é uma chamada de ferramenta enfileirada que requer que um humano aprove ou rejeite antes que a plataforma a execute.

### Configurando aprovações

No formulário de edição do agente, a seção **Approvals** lista todas as ferramentas que o agente tem permissão para invocar (a allowlist) e permite marcar aquelas que devem ser revisadas por um humano. Ferramentas não marcadas são executadas imediatamente. Ferramentas marcadas são enfileiradas.

Ferramentas não permitidas são *recusadas imediatamente*, não enfileiradas - aprovações só se aplicam a ferramentas que, de outra forma, são permitidas.

### O que acontece quando uma ação com gate é disparada

1. O agente seleciona uma chamada de ferramenta (ex.: `ban_user`) com argumentos, justificativa e confiança.
2. Em vez de executar, a plataforma enfileira uma aprovação em estado `PENDING` com o nome da ferramenta, argumentos, justificativa, confiança e um snapshot de contexto descrevendo o gatilho que a produziu.
3. Notificações são enviadas aos revisores (veja [Notificações de Aprovação](#approval-notifications)).
4. A execução do agente é concluída e registrada - a ação é exibida com **Pending approval** em [Visualização de Detalhes da Execução](#run-detail-view).

### Revisando aprovações

A caixa de entrada de aprovações em [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) lista aprovações pendentes, aprovadas, rejeitadas e com falha de execução. Para cada uma:

- **Nome da ferramenta e argumentos** - exatamente o que o agente quer fazer.
- **Raciocínio do agente** - a justificativa fornecida pelo agente.
- **Confiança** - a autoconfiança do agente, de 0.0 a 1.0.
- **Snapshot de contexto** - o comentário, a página e o usuário que são alvo da ação.

Dois botões: **Aprovar** e **Rejeitar**. Aprovar executa realmente a ferramenta; Rejeitar descarta.

### Estados de status da aprovação

Uma aprovação passa por estes estados:

| State | Meaning |
|---|---|
| `PENDING` | Aguardando decisão humana. |
| `APPROVED` | Um humano aprovou e a ação foi executada. |
| `REJECTED` | Um humano rejeitou. A ação não foi executada. |
| `EXECUTION_FAILED` | Um humano aprovou, mas a execução falhou (ex.: o comentário alvo já foi excluído). |
| `EXECUTING` | Transitório: um humano clicou em Approve e a ação está em execução. Usado para serializar cliques simultâneos em approve para que uma ferramenta com efeitos colaterais reais nunca rode duas vezes. |

O estado `EXECUTING` importa quando dois revisores clicam em Approve simultaneamente - um vence, o outro vê que a aprovação já mudou de estado.

### O que é limpo/expurgado

As aprovações pendentes permanecem pendentes até serem decididas. Elas expiram automaticamente depois de **90 dias** da criação. Aprovações em qualquer outro estado também são limpas após 90 dias por motivos de higiene de armazenamento.

Os campos "decidido por" / "decidido em" / "executado em" / "resultado da execução" da aprovação são preenchidos conforme a aprovação avança pelos estados - todos visíveis na visualização detalhada da caixa de entrada.

### Integração via webhook

Dois eventos de webhook disparam conforme as aprovações se movem:

- **`approval.requested`** - na inserção PENDING.
- **`approval.decided`** - na transição para APPROVED, REJECTED, ou EXECUTION_FAILED.

Ambos são assinados como qualquer outro webhook. Veja [Webhook Events](#webhook-events) e [Webhook Payloads](#webhook-payloads).

### Endurecimento: gate de ferramenta conhecida

As aprovações recusam enfileirar qualquer nome de ferramenta que não seja uma ferramenta de agente reconhecida. Esta é uma verificação de defesa em profundidade: mesmo que um caminho de código futuro passe um nome de ferramenta derivado de LLM para o fluxo de aprovação, essa string nunca poderá chegar a uma aprovação enfileirada. O conjunto de nomes de ferramentas conhecidas é o mesmo conjunto listado em [Tools Reference](#tools-overview).

### Padrões comuns de gating

- **Agente de moderação novo** - aplique gating em `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Observe a caixa de entrada por algumas semanas e então remova o gating das ferramentas com poucos erros.
- **Agente de moderação de longo prazo** - mantenha `ban_user` e quaisquer ações irreversíveis (`deleteAllUsersComments`, `banIP`) sempre com gate.
- **Região UE** - `ban_user` é forçado por Article 17 independentemente do que você marque. Veja [Conformidade com o Artigo 17 do DSA da UE](#eu-dsa-compliance).

### O que as aprovações **não** fazem

- Elas não atrasam as outras chamadas de ferramenta do agente. A execução do agente pode chamar várias ferramentas, e apenas as com gate são enfileiradas - o restante executa normalmente.
- Elas não desfazem a execução do agente se o humano rejeitar. A parte não gate da execução já foi concluída.