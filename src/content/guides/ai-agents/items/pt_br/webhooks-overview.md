Agent webhooks são callbacks HTTP acionados pela plataforma quando uma execução de agente é concluída ou uma aprovação muda de estado. Use-os para encaminhar a atividade do agente para seus próprios sistemas - painéis de moderação, logs de auditoria, canais do Slack, ferramentas de escalonamento.

Configurados na aba **Webhooks** na [página AI Agents](https://fastcomments.com/auth/my-account/ai-agents).

### O que é entregue

Quatro tipos de evento:

- **`trigger.succeeded`** - uma execução do agente foi concluída com sucesso.
- **`trigger.failed`** - uma execução do agente apresentou erro.
- **`approval.requested`** - uma ação foi enfileirada para aprovação humana.
- **`approval.decided`** - uma aprovação foi aprovada, rejeitada, ou a execução falhou.

Veja [Webhook Events](#webhook-events) para saber quando cada evento é acionado, e [Webhook Payloads](#webhook-payloads) para o esquema de cada um.

### O que não é entregue

- **Webhooks por ação de ferramenta.** Uma execução que chama `pin_comment` não dispara um webhook separado para o pin - a ação é incluída no payload `trigger.succeeded` da execução. Se você deseja entrega por ação, analise o array `actions` no payload do trigger.
- **Triggers descartados.** Um trigger que não é disparado (por exceder orçamento, escopo incorreto) não gera webhook. Triggers descartados são visíveis apenas na [página Analytics](#analytics-page).
- **Triggers produzidos por replay.** Execuções de teste não disparam webhooks. Veja [Execuções de teste (Replays)](#test-runs-replays).

### Configuração

Para cada webhook que você configurar:

- **URL** - o endpoint HTTPS para POST.
- **Domain** - o domínio de comentários ao qual este webhook se aplica (seu tenant pode hospedar múltiplos domínios). `*` corresponde a todos os domínios; `*.example.com` é um glob; um domínio exato corresponde somente àquele.
- **Events** - quais dos quatro tipos de evento assinar.
- **Agents** - vazio significa 'todos os agentes', ou uma lista de IDs de agentes específicos para filtrar.
- **Method** - POST ou PUT (padrão POST).
- **No-retry status codes** - códigos de resposta HTTP que devem ser tratados como falhas terminais, não re-tentadas (ex.: 410 Gone, 422 Unprocessable). Veja [Webhook Retries](#webhook-retries).

Vários webhooks podem assinar o mesmo evento - cada um enfileira de forma independente e é entregue para sua própria URL.

### Correspondência por domínio

Um evento é entregue a **todo webhook cujo campo `domain` corresponda ao domínio do comentário**. A correspondência usa um glob simples:

- Exato: `example.com` corresponde apenas a `example.com`.
- Asterisco coringa: `*` corresponde a todos os domínios.
- Glob de subdomínio: `*.example.com` corresponde a `blog.example.com`, `forum.example.com`, mas não a `example.com`.

O domínio em um payload é o primeiro resultado não-nulo de: o `domain` do comentário, a URL analisada contra a configuração de domínio do seu tenant, ou a busca `Page` pelo `urlId`.

### Filtragem por agente

O campo **Agents** permite que um webhook se inscreva apenas em determinados agentes. Vazio significa 'todos os agentes'. Quando não vazio, o webhook só é acionado para os agentes da lista.

Isto é útil quando você tem um webhook para eventos de moderação e outro para eventos de engajamento, ambos roteando para diferentes sistemas downstream.

### Envio de teste

A UI de configuração de webhook tem um botão **Envio de teste** que envia um payload falso para a URL para que você possa verificar conectividade, assinatura e o código de resposta do seu endpoint sem esperar por um evento real.

### Logs de entrega

Cada entrega (e cada re-tentativa) aparece na página [Logs de entrega do webhook](#webhook-logs) para que você possa ver o que aconteceu na comunicação.

### Assinatura

Todo webhook é assinado com HMAC-SHA256 usando o segredo de API do seu tenant. Veja [Assinatura do webhook](#webhook-signing).

### Elegibilidade

Webhooks de agente exigem cobrança válida no tenant. Eles usam a mesma infraestrutura de assinatura/segredo que seus webhooks de comentário existentes - se você já integrou webhooks de comentário (veja o [Guia de Webhooks](/guide-webhooks.html)), a integração de webhooks de agente tem a mesma forma com novos tipos de evento.