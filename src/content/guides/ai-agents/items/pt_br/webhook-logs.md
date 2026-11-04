Cada webhook de agente tem seu próprio log de entrega. Acessível pela [página de lista de webhooks](https://fastcomments.com/auth/my-account/ai-agents/webhooks) através do botão **Logs** em cada linha de webhook.

### O que há na página

Uma tabela paginada com uma linha por tentativa de entrega:

| Coluna | Significado |
|---|---|
| When | Quando a tentativa aconteceu. |
| Event | O tipo de evento (`trigger.succeeded`, `approval.requested`, etc.). |
| Status | O status da entrega. |
| StatusCode | Código de status HTTP retornado pelo seu endpoint, quando disponível. |
| Description | Uma breve descrição do resultado. Para entregas com falha onde nenhuma resposta HTTP foi recebida, a mensagem de erro subjacente é armazenada como `{error: <message>}`. |

A página suporta apenas paginação - não há filtros de status, tipo de evento ou intervalo de datas.

### O que você pode fazer a partir dos logs

- **Decidir se um código de status deve estar em No-retry.** Se você vê seu endpoint retornando o mesmo `4xx` repetidamente, esse é um forte sinal de que você deseja adicioná-lo aos **No-retry status codes** para que a plataforma pare de tentar novamente.

### Informações de falha

Quando uma entrega falha sem uma resposta HTTP (falha de DNS, conexão recusada, timeout, erro de TLS, etc.), a mensagem de erro bruta é registrada como `{error: <message>}`. A plataforma não categoriza esses erros em buckets nomeados como `TIMEOUT` ou `DNS_ERROR` - leia a mensagem de erro diretamente para ver o que aconteceu.

Para respostas HTTP, a coluna StatusCode mostra o código retornado pelo seu endpoint. Casos comuns:

- **Todas as tentativas: `401` ou `403`** - seu endpoint está rejeitando a assinatura. Verifique se você está calculando o HMAC sobre `${timestamp}.${body}` e usando o segredo correto. Veja [Assinatura de Webhook](#webhook-signing).
- **Todas as tentativas: `422`** - seu endpoint considera o payload inválido. Ou corrija seu endpoint, ou adicione `422` aos **No-retry status codes** se a rejeição for esperada para alguns eventos.

### Contexto por entrega

Cada entrada de log contém:

- `webhookId` - qual configuração de webhook produziu esta entrega.
- `agentId` - a qual agente a entrega se refere.
- `triggerId` ou `approvalId` - o registro subjacente.
- `domain` - o domínio correspondente.

Você pode usar esses dados para correlacionar uma entrega com falha à execução à qual ela se relaciona no [Histórico de Execuções](#run-history).

### Retenção

As entradas de `AgentSyncLog` têm um TTL fixo de 1 ano em `createdAt`, independentemente do resultado - entregas bem-sucedidas e com falha são retidas pelo mesmo período de tempo. Após a retenção, a entrada de log é removida.

Se você precisa de auditoria de longo prazo, o padrão sustentável é: fazer com que o **próprio endpoint** persista as entregas que recebe. Isso desacopla seu log de auditoria da política de retenção da plataforma.

### Envio de teste

O botão **Test send** do formulário de configuração do webhook grava uma entrega falsa na mesma tabela de log para que você possa verificar a conectividade de ponta a ponta antes de depender de eventos reais. As entregas de teste são claramente rotuladas no log para que não poluam as estatísticas de falhas de produção.

### Veja também

- [Visão Geral de Webhooks](#webhooks-overview).
- [Retentativas de Webhook](#webhook-retries) para a semântica de retentativas que gera esses logs.
- [Assinatura de Webhook](#webhook-signing) para como verificar no seu endpoint.