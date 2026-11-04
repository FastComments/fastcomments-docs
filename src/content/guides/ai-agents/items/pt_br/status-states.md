Um agente tem um de três status:

### Disabled

O agente está desligado. Nenhum trigger é processado e o agente não aparece no caminho de despacho. Seu histórico de execuções, análises e memória permanecem - se você reativá-lo mais tarde, os dados históricos ainda estarão lá.

Use `Disabled` quando:
- Você quer tirar um agente de rotação sem perdê-lo.
- Um agente está se comportando mal e você precisa pará-lo imediatamente enquanto investiga.
- Você está alternando agentes sazonalmente (ex.: um saudador apenas para feriados).

### Dry Run - padrão para novos agentes

O agente roda de ponta a ponta - ele processa triggers, chama o LLM, escolhe chamadas de ferramentas, calcula justificativas e confiança - mas **nenhuma ação real é tomada**. Cada execução é registrada com o selo **Dry Run** em [Run History](#run-history).

Use `Dry Run` quando:
- Um agente novo acabou de sair da caixa. Todo template inicial começa em dry-run.
- Você editou o prompt ou alterou o conjunto de triggers e quer ver como a mudança se comporta antes de confirmar.
- Você está executando uma [execução de teste / replay](#test-runs-replays) (replays forçam dry-run independentemente do status do agente).

A plataforma cobra tokens por execuções em dry-run - a chamada ao LLM ainda acontece, apenas os efeitos colaterais são ignorados. Limites de orçamento também se aplicam ao dry-run. Veja [Budgets Overview](#budgets-overview).

### Enabled

O agente executa ações reais. Chamadas de ferramentas são executadas - ou entram na fila para [aprovação](#approval-workflow) se a ação for controlada.

Use `Enabled` depois que a saída do dry-run parecer correta.

### Alternando status

Você pode alternar entre quaisquer dois status no formulário de edição. Alternar de Dry Run para Enabled não reexecuta retroativamente as ações do dry-run - elas permanecem como histórico de dry-run. Novos triggers a partir desse momento rodam em produção.

Alternar de Enabled para Disabled no meio de uma execução **não** aborta uma execução em andamento. O trigger em execução atual termina (com o que já tiver iniciado); o próximo trigger é descartado porque o agente agora está Disabled.

### Status durante problemas de cobrança

Se o faturamento do seu tenant se tornar inválido, todos os agentes são efetivamente pausados independentemente do status salvo - os triggers são descartados com `BILLING_INVALID` até que o faturamento seja restaurado. O campo de status salvo não é alterado; o dispatcher simplesmente se recusa a executar. Veja [Plans and Eligibility](#plans-and-eligibility).