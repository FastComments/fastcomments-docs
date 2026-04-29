Run History é o registro por agente de cada trigger que foi executado. Acessível a partir da página de lista de agentes pelo botão **Runs**, ou diretamente em `/auth/my-account/ai-agents/{agentId}/runs`.

### O que há na página

Uma tabela paginada com uma linha por execução:

| Coluna | Significado |
|---|---|
| Date | Quando o trigger disparou (ou quando o trigger adiado foi executado). |
| Status | **Started**, **Success**, or **Error**. Uma insígnia **Dry Run** é exibida ao lado se a execução estiver em modo dry-run. |
| Cost | Custo por execução na moeda do seu tenant. Vazio para execuções em andamento (Started). |
| Actions | O número de chamadas de ferramentas na execução. |
| Details | Um botão **View** que abre [Visualização de Detalhes da Execução](#run-detail-view). |

### Significados dos status

- **Started** - a execução está em andamento, ou morreu antes de ser concluída. Uma execução travada em "Started" por um tempo anormalmente longo geralmente representa um timeout de chamada ao LLM.
- **Error** - a execução foi concluída, mas falhou em algum ponto - a chamada ao LLM retornou um erro, o despacho de uma ferramenta falhou, etc. A visualização de detalhes contém o erro específico.
- **Success** - a execução foi concluída sem erro. O agente pode ter tomado zero, uma, ou várias ações.

### Estado vazio

Quando um agente não tem execuções, a página mostra: "Ainda não há execuções para este agente. Execuções habilitadas aparecem aqui assim que um trigger for acionado; use Execução de teste para visualizar o que este agente faria com comentários passados."

Essa última parte é intencional - o [fluxo de execução de teste](#test-runs-replays) é a maneira recomendada de preencher o Histórico de Execuções em um agente novo.

### O que não está na página de histórico de execuções

- **Live triggers that never dispatched** - um trigger descartado por orçamento, escopo ou limitação de taxa não aparece nesta página. Eles aparecem na [página de Analytics](#analytics-page) em "Triggers skipped".
- **Approvals** - aprovações pendentes para ações tomadas nesta execução ficam na [caixa de entrada de aprovações](#approval-workflow). A ação aparece na visualização de detalhes da execução como **Pendente de aprovação**.

### Retenção

Os registros individuais de execução são mantidos por 90 dias, após os quais a execução desaparece do histórico. Custo e contagens de trigger continuam sendo agregados em resumos analíticos de longo prazo, então a [página de Analytics](#analytics-page) ainda mostra totais históricos além desse período.

### Replays

Execuções produzidas por replay são excluídas da visualização de execuções ao vivo por padrão. A página [Execuções de Teste (Replays)](#test-runs-replays) é onde você as vê.

### Filtragem entre agentes

A tabela de execuções é por agente. Não existe uma visualização de execuções entre agentes - a [página de Analytics](#analytics-page) é o resumo entre agentes. Se você precisa inspecionar execuções em vários agentes, os eventos do [Webhooks](#webhooks-overview) `trigger.succeeded` and `trigger.failed` são aqueles que você deve encaminhar para o seu próprio sistema.

---