Um agente de codificação, como Claude Code, Cursor ou um assistente baseado em MCP, pode configurar o FastComments para você sem que você preencha o formulário de cadastro. Isso é útil quando você pede a um agente para "adicionar comentários ao meu site" e ainda não tem uma conta.

### Como funciona

1. O agente cria uma nova conta e recebe uma chave de API e um link de reivindicação. A chave de API funciona imediatamente, então o agente pode configurar a conta e instalar o widget no seu site.
2. O agente lhe fornece o link de reivindicação. Abra-o no seu navegador, faça login ou crie um login e confirme a reivindicação. A conta então é sua: você a gerencia, junto com seu faturamento e suas chaves de API, pelo painel. A página lista a chave de API que o agente possui, para que você possa revogá-la se não quiser mais que o agente, ou quem o opera, tenha acesso.
3. Se ninguém abrir o link de reivindicação em até 72 horas, a conta e sua chave são excluídas. Peça ao agente para criar uma nova.

Até ser reivindicada, a conta tem os mesmos limites de uma avaliação gratuita normal.

### Se você já tem uma conta

Cada login possui uma conta. Se você abrir um link de reivindicação enquanto estiver logado em uma conta existente, a página permite escolher:

- **Vincular à minha conta** torna a nova conta um tenant gerenciado daquela em que você está logado. Isso requer um plano pago com white labeling, e o uso do novo tenant é faturado na sua conta.
- **Sair e reivindicar com outro login** desconecta você e leva de volta à página de reivindicação para que você possa reivindicá-la com um login diferente.

### Para autores de agentes

As instruções do agente em [fastcomments.com/agents.md](https://fastcomments.com/agents.md) descrevem a chamada de criação de conta, os campos na resposta e como entregar o link de reivindicação à pessoa para quem você está trabalhando. A chamada não precisa de chave de API e é limitada por taxa por endereço IP.