---
Por padrão um agente é executado **imediatamente** depois que seu gatilho dispara. O campo **Delay before running** no formulário de edição altera isso: a plataforma enfileira o gatilho e executa o agente no horário agendado.

### Quando usar um atraso

- **Gatilhos por limiar de sinalizações** - as sinalizações muitas vezes chegam em rajadas. Um atraso de 10-30 minutos permite que a situação se estabilize, de modo que o agente aja sobre a contagem final de sinalizações em vez do momento de chegada.
- **Gatilhos por limiar de votos** - mesma lógica, particularmente para ataques em massa de votos negativos.
- **Resumo de tópico** - o [Thread Summarizer template](#template-thread-summarizer) tem por padrão um atraso de 30 minutos para que ele resuma uma conversa que teve tempo para se desenvolver, e não um tópico com apenas duas respostas.
- **Período de espera / reavaliação** - "24 horas após um comentário ser bloqueado, considere se deve desbloqueá-lo."

### Configuração

- **Field**: Delay before running.
- **Range**: 0 a 2,592,000 segundos (30 dias).
- **Units**: Segundos, minutos, horas ou dias.

### Idempotência

A fila diferida não remove duplicatas de gatilhos. Duas sinalizações chegando com 1 segundo de diferença em um agente com atraso de 30 minutos irão agendar ambas uma execução 30 minutos depois, e o agente será executado **duas vezes**, ambas as vezes contra (principalmente) o mesmo contexto. Se você quiser semântica de no-máximo-uma-execução-por-janela, o agente precisa aplicá-la — tipicamente escrevendo uma [memory note](#tools-overview) na primeira execução e verificando-a nas execuções subsequentes.

### Observação sobre custo

Os gatilhos diferidos são registrados **antes** de serem executados. Um pico de gatilhos em um agente com alto atraso pode se acumular na fila sem gastar tokens; o custo é pago somente quando o cron os despacha. Use [Run History](#run-history) e [Drop Reasons](#drop-reasons) para ver com que frequência os gatilhos diferidos realmente são executados vs. são descartados em tempo de execução por motivos de orçamento.

### Replay não respeita o atraso

O recurso [Test Runs (Replays)](#test-runs-replays) executa o agente imediatamente contra comentários históricos — ele não espera pelo atraso configurado. Considere isso um recurso: replays servem para visualizar o que o agente **faria** dado o contexto, não para reproduzir o agendamento em tempo real.

---