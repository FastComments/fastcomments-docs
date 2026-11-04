Os webhooks de agente tentam novamente em caso de falha. A entrega é **fire-and-forget da perspectiva do agente** - uma entrega com falha não bloqueia a execução do agente nem desfaz qualquer ação - e uma fila + cron gerencia as novas tentativas de forma assíncrona.

### Modelo de fila

Cada evento é enfileirado **uma vez por webhook correspondente**. Então, se você tiver três webhooks inscritos em `trigger.succeeded` para um determinado agente + domínio, a plataforma enfileira três entregas; cada uma é entregue e tentada novamente de forma independente. Uma falha em um webhook nunca afeta os outros.

### O que é tentado novamente

Uma entrega é tentada novamente quando:

- A requisição HTTP **não é concluída** (falha de DNS, conexão recusada, timeout).
- O código de resposta HTTP é qualquer status não-2xx que não esteja na lista configurada de **No-retry status codes**.

Uma entrega **não é tentada novamente** quando:

- O código de resposta é `2xx` (sucesso).
- O código de resposta está na lista configurada de **No-retry status codes**. Por padrão, essa lista está vazia - qualquer não-2xx é tentado novamente.

### Configurando códigos sem nova tentativa

O formulário de configuração do webhook tem um campo **No-retry status codes** (multi-valor). Entradas comuns:

- `410` - Gone. Seu endpoint foi movido permanentemente ou o recurso não existe mais. Tentar novamente apenas desperdiça a largura de banda de ambos os lados.
- `422` - Unprocessable Entity. Seu endpoint entendeu o payload, mas o considerou inválido. Tentar novamente com o mesmo payload retornará a mesma resposta.
- `400` - Bad Request, no mesmo espírito.

Adicionar um código aqui significa: quando o endpoint o retornar, marcar a entrega como falha-terminal e parar de tentar novamente.

### Cronograma de novas tentativas

Um worker em segundo plano roda a cada poucos segundos e processa todas as entregas cujo horário da próxima tentativa já passou.

Após cada falha, o horário da próxima tentativa é adiado com **backoff linear**: a espera cresce como `60 segundos * contagem de tentativas` (então a tentativa 1 espera 1 minuto, a tentativa 2 espera 2 minutos, e assim por diante).

Após 99 tentativas com falha (ou 3 em desenvolvimento local), a entrega é abandonada e removida da fila. As entradas do log de entrega persistem e permanecem visíveis na página [Webhook Delivery Logs](#webhook-logs) até expirarem.

### Idempotência do seu lado

Como fazemos novas tentativas, seu endpoint **deve ser idempotente**. O mesmo `triggerId` (ou `approvalId`) pode chegar mais de uma vez. Seu endpoint deve:

- Usar uma chave única (`triggerId` para eventos de trigger, `approvalId` para eventos de approval) como token de deduplicação.
- Aceitar entregas duplicadas de forma graciosa (retornar 200 na segunda vez).

Um endpoint não idempotente eventualmente processará em duplicidade algumas entregas, especialmente durante interrupções transitórias onde um timeout tenta novamente 30 segundos depois, mas a requisição original na verdade teve sucesso.

### Ordenação

As entregas **não são estritamente ordenadas**. Um `trigger.succeeded` e um `approval.requested` downstream (da mesma execução) podem chegar em qualquer ordem se um tentar novamente e o outro não. Seu endpoint não deve assumir ordenação causal.

Se você precisar de ordenação, use os timestamps - `occurredAt` no envelope, mais o `createdAt` do trigger/approval no bloco de dados - para reconstruir a ordem do seu lado.

### Limpeza

As entregas são removidas da fila assim que têm sucesso ou atingem o limite de tentativas. A plataforma não retém entregas com falha terminal na própria fila; o registro durável de cada tentativa fica na página [Webhook Delivery Logs](#webhook-logs).

### Onde verificar quando as novas tentativas falham

A página [Webhook Delivery Logs](#webhook-logs) é o lugar para ver por que um webhook está falhando. Causas comuns:

- **Falha de resolução de DNS** - a URL está errada ou o domínio não existe mais.
- **Erros de TLS** - o certificado do seu endpoint é inválido ou expirou.
- **Conexão recusada / timeout** - seu endpoint está fora do ar.
- **Respostas 5xx** - seu endpoint está no ar, mas retornou erro. O corpo da resposta (truncado) é registrado.
- **Respostas 4xx** - seu endpoint rejeitou o payload. Se isso for intencional, adicione o código a **No-retry status codes**.

### Pausar um webhook não saudável

Se um webhook está falhando consistentemente, a solução mais limpa é excluí-lo (ou limpar temporariamente sua lista de inscrições de eventos). A plataforma não desativa automaticamente webhooks com falha - eles continuam tentando até que a entrega seja abandonada.