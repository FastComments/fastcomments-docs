Um agente tem um de três status:

### Disabled

O agente está desligado. Nenhum gatilho é processado e o agente não aparece no caminho de despacho. Seu histórico de execução, análises e memória permanecem - se você reativá-lo mais tarde, os dados históricos ainda estarão lá.

Use `Disabled` quando:
- Você quiser tirar um agente de rotação sem perdê-lo.
- Um agente estiver se comportando mal e você precisar interrompê-lo imediatamente enquanto investiga.
- Você estiver alternando agentes sazonalmente (ex.: um recepcionista apenas para feriados).

### Dry Run - padrão para novos agentes

O agente é executado de ponta a ponta - processa gatilhos, chama o LLM, escolhe chamadas de ferramenta, calcula justificativas e confiança - mas **nenhuma ação real é tomada**. Cada execução é registrada com o selo **Dry Run** no [Histórico de Execução](#run-history).

Use `Dry Run` quando:
- Um novo agente acabou de sair da caixa. Todo modelo inicial entra em dry-run.
- Você editou o prompt ou alterou o conjunto de gatilhos e quer ver como a mudança se comporta antes de confirmar.
- Você está executando uma [execução de teste / reprodução](#test-runs-replays) (replays forçam dry-run independentemente do status do agente).

A plataforma cobra tokens por execuções em dry-run - a chamada ao LLM ainda ocorre, apenas os efeitos colaterais são pulados. Limites de orçamento também se aplicam ao dry-run. Veja [Visão Geral de Orçamentos](#budgets-overview).

### Enabled

O agente realiza ações reais. Chamadas de ferramenta são executadas - ou entram na fila para [aprovação](#approval-workflow) se a ação for condicionada.

Use `Enabled` após a saída do dry-run parecer correta.

### Switching status

Você pode alternar entre quaisquer dois status no formulário de edição. Mudar de Dry Run para Enabled não reexecuta retroativamente as ações do dry-run - essas permanecem como histórico de dry-run. Novos gatilhos a partir desse momento são executados ao vivo.

Mudar de Enabled para Disabled no meio de uma execução **não** aborta uma execução em andamento. O gatilho atualmente em execução termina (com o que já tiver iniciado); o próximo gatilho é descartado porque o agente agora está Disabled.

### Status durante problemas de faturamento

Se o faturamento do seu tenant ficar inválido, todos os agentes ficam efetivamente pausados independentemente do status salvo - gatilhos são descartados com `BILLING_INVALID` até que o faturamento seja restaurado. O campo de status salvo não é alterado; o despachante apenas se recusa a executar. Veja [Planos e Elegibilidade](#plans-and-eligibility).