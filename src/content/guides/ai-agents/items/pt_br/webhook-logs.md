Cada webhook de agente tem seu próprio registro de entregas. Acessível a partir da [página de lista de webhooks](https://fastcomments.com/auth/my-account/ai-agents/webhooks) via o botão **Registros** em cada linha do webhook.

### O que há na página

Uma tabela paginada com uma linha por tentativa de entrega:

| Column | Meaning |
|---|---|
| When | Quando a tentativa aconteceu. |
| Event | O tipo de evento (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | O status da entrega. |
| StatusCode | Código de status HTTP retornado pelo seu endpoint, quando disponível. |
| Description | Uma breve descrição do resultado. Para entregas com falha em que nenhuma resposta HTTP foi recebida, a mensagem de erro subjacente é armazenada como `{error: <message>}`. |

A página suporta apenas paginação - não há filtros por status, tipo de evento ou intervalo de datas.

### O que você pode fazer nos logs

- **Decidir se um código de status deve entrar em No-retry.** Se você ver seu endpoint retornando o mesmo `4xx` repetidamente, isso é um forte indicador de que você deve adicioná-lo aos **Códigos de status sem nova tentativa** para que a plataforma pare de tentar novamente.

### Informações sobre falhas

Quando uma entrega falha sem uma resposta HTTP (falha de DNS, conexão recusada, timeout, erro TLS, etc.), a mensagem de erro bruta é registrada como `{error: <message>}`. A plataforma não categoriza esses erros em categorias nomeadas como `TIMEOUT` ou `DNS_ERROR` - leia a mensagem de erro diretamente para ver o que aconteceu.

Para respostas HTTP, a coluna StatusCode mostra o código retornado pelo seu endpoint. Casos comuns:

- **All attempts: `401` or `403`** - seu endpoint está rejeitando a assinatura. Verifique se você está calculando o HMAC sobre `${timestamp}.${body}` e usando o segredo correto. Veja [Assinatura de Webhook](#webhook-signing).
- **All attempts: `422`** - seu endpoint acha que o payload é inválido. Ou corrija seu endpoint, ou adicione `422` aos **Códigos de status sem nova tentativa** se a rejeição for esperada para alguns eventos.

### Contexto por entrega

Cada entrada de log contém:

- `webhookId` - qual configuração de webhook gerou essa entrega.
- `agentId` - sobre qual agente a entrega é.
- `triggerId` ou `approvalId` - o registro subjacente.
- `domain` - o domínio correspondente.

Você pode usar esses dados para correlacionar uma entrega com falha à execução a que ela se relaciona em [Histórico de execuções](#run-history).

### Retenção

`AgentSyncLog` entries have a flat 1-year TTL on `createdAt` regardless of outcome - successful and failed deliveries are retained for the same length of time. Beyond retention, the log entry is gone.

Se você precisa de auditoria a longo prazo, o padrão sustentável é: faça com que o próprio endpoint persista as entregas que recebe. Isso desacopla seu log de auditoria da política de retenção da plataforma.

### Envio de teste

O botão **Envio de teste** no formulário de configuração do webhook grava uma entrega falsa na mesma tabela de logs para que você possa verificar a conectividade de ponta a ponta antes de depender de eventos reais. Entregas de teste são claramente identificadas no log para que não poluam as estatísticas de falhas de produção.

### Veja também

- [Visão geral de Webhooks](#webhooks-overview).
- [Retentativas de Webhook](#webhook-retries) para a semântica de reenvio que alimenta esses logs.
- [Assinatura de Webhook](#webhook-signing) para como verificar no seu endpoint.