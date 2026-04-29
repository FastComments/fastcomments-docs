Agent webhooks tentam novamente em caso de falha. A entrega é **fire-and-forget da perspectiva do agente** - uma entrega fracassada não bloqueia a execução do agente nem reverte quaisquer ações - e uma fila + cron impulsionam as tentativas de forma assíncrona.

### Queue model

Cada evento é enfileirado **uma vez por webhook correspondente**. Então, se você tiver três webhooks inscritos em `trigger.succeeded` para um determinado agente + domínio, a plataforma enfileira três entregas; cada uma é entregue e retentada de forma independente. Uma falha em um webhook nunca afeta os outros.

### What's retried

Uma entrega é tentada novamente quando:

- A requisição HTTP **não é concluída** (falha de DNS, conexão recusada, timeout).
- O código de resposta HTTP é qualquer status não 2xx que não esteja na lista configurada de **No-retry status codes**.

Uma entrega **não é tentada novamente** quando:

- O código de resposta é `2xx` (sucesso).
- O código de resposta está na lista configurada de **No-retry status codes**. Por padrão essa lista está vazia - qualquer não-2xx é retentado.

### Configuring no-retry codes

O formulário de configuração do webhook tem um campo **No-retry status codes** (multivalor). Entradas comuns:

- `410` - Gone. Seu endpoint foi movido permanentemente ou o recurso não existe mais. Retentar apenas desperdiça largura de banda de ambos os lados.
- `422` - Unprocessable Entity. Seu endpoint entendeu o payload mas o considerou inválido. Retentar com o mesmo payload dará a mesma resposta.
- `400` - Bad Request, no mesmo espírito.

Adicionar um código aqui significa: quando o endpoint o retornar, marcar a entrega como failed-terminal e parar de retentar.

### Retry schedule

Um worker em background roda a cada poucos segundos e processa quaisquer entregas cujo próximo horário de tentativa já passou.

Após cada falha, o tempo do próximo-attempt é empurrado para frente com **backoff linear**: a espera cresce como `60 seconds * attempt count` (então a tentativa 1 espera 1 minuto, a tentativa 2 espera 2 minutos, e assim por diante).

Após 99 tentativas falhas (ou 3 em desenvolvimento local), a entrega é abandonada e removida da fila. As entradas do log de entrega persistem e permanecem visíveis na página [Registros de Entrega de Webhook](#webhook-logs) até expirarem.

### Idempotence on your side

Como re-tentamos, seu endpoint **deve ser idempotente**. O mesmo `triggerId` (ou `approvalId`) pode chegar mais de uma vez. Seu endpoint deve:

- Usar uma chave única (`triggerId` para eventos de trigger, `approvalId` para eventos de aprovação) como token de deduplicação.
- Aceitar entregas duplicadas graciosamente (retornar 200 na segunda vez).

Um endpoint não idempotente eventualmente processará em dobro algumas entregas, especialmente durante interrupções transitórias onde um timeout re-tenta 30 segundos depois, mas a requisição original na verdade teve sucesso.

### Ordering

As entregas **não são estritamente ordenadas**. Um `trigger.succeeded` e um `approval.requested` subsequente (da mesma execução) podem chegar em qualquer ordem se um re-tentar e o outro não. Seu endpoint não deve assumir ordenação causal.

Se você precisa de ordenação, use os timestamps - `occurredAt` no envelope, além do `createdAt` do trigger/aprovação no bloco de dados - para reconstruir a ordem do seu lado.

### Cleanup

As entregas são removidas da fila assim que ou têm sucesso ou atingem o limite de tentativas. A plataforma não retém entregas terminalmente falhas na própria fila; o registro durável de cada tentativa fica na página [Registros de Entrega de Webhook](#webhook-logs).

### Onde procurar quando as tentativas falham

A página [Registros de Entrega de Webhook](#webhook-logs) é o lugar para ver por que um webhook está falhando. Causas comuns:

- **Falha de resolução de DNS** - a URL está errada ou o domínio não existe mais.
- **Erros de TLS** - o certificado do seu endpoint é inválido ou expirado.
- **Conexão recusada / timeout** - seu endpoint está fora do ar.
- **Respostas 5xx** - seu endpoint está ativo mas retornou erro. O corpo da resposta (truncado) é registrado.
- **Respostas 4xx** - seu endpoint rejeitou o payload. Se isso for intencional, adicione o código em **No-retry status codes**.

### Pause an unhealthy webhook

Se um webhook estiver falhando consistentemente, a solução mais limpa é excluí-lo (ou limpar temporariamente sua lista de assinaturas de eventos). A plataforma não desativa automaticamente webhooks que falham - eles continuam retentando até que a entrega seja abandonada.