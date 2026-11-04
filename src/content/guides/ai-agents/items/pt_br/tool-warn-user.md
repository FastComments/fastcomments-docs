A ferramenta Warn envia um aviso privado por DM a um usuário sobre um comentário específico e, ao mesmo tempo, registra o aviso na [memória do agente](#agent-memory-system) compartilhada. As duas gravações são atômicas - o usuário nunca vê um aviso que não esteja também registrado.

### Por que ela existe

A política de escalonamento da plataforma é **avisar primeiro, banir apenas se o usuário reincidir**. A ferramenta Warn é o que torna essa política acionável: ela dá ao usuário a chance de corrigir o rumo, e o registro do aviso é o que um agente futuro encontra ao pesquisar na memória antes de considerar um banimento.

A ferramenta também elimina duplicatas: se o agente já emitiu um aviso ao mesmo usuário sobre o mesmo comentário, um segundo aviso é uma no-op. Assim, um LLM que entra em loop ou dispara novamente no mesmo comentário não pode bombardear o usuário com múltiplos avisos.

### O que vai no aviso

Uma mensagem curta (limitada a 1000 caracteres) exibida ao usuário como uma DM. Avisos eficazes são:

- **Específicos** - "Ataques pessoais a usuários nomeados não são permitidos nesta comunidade" é melhor do que "seu comentário foi sinalizado".
- **Curtos** - no máximo algumas frases.
- **Acionáveis** - diga ao usuário o que mudar. "Por favor, edite seu comentário para remover o usuário nomeado, ou ele será removido."

Você não escreve a mensagem você mesmo; o agente faz isso, com base no [prompt inicial](#personality-prompt) e nas [diretrizes da comunidade](#community-guidelines). Seu trabalho é escrever um prompt que produza bons avisos.

### Quando permitir

Para qualquer agente de moderação. O template Moderator a habilita por padrão.

### Aprovações

Menos comumente controlada do que [Ban user](#tool-ban-user). Vale a pena controlar durante as primeiras semanas de vida de um agente para que você possa identificar avisos ruins antes que sejam enviados, mas a maioria dos operadores remove o controle assim que o agente está produzindo resultados confiáveis.

### Veja também

- [Ban user](#tool-ban-user) - o próximo passo no escalonamento.
- [Agent Memory System](#agent-memory-system) - onde os registros de avisos ficam.