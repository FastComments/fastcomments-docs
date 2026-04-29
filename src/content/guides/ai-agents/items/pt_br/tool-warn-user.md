A ferramenta Warn envia um aviso por mensagem direta (DM) privada a um usuário sobre um comentário específico e, ao mesmo tempo, registra o aviso na [memória de agente](#agent-memory-system) compartilhada. As duas gravações são atômicas — o usuário nunca vê um aviso que não esteja também registrado.

### Why it exists

A política de escalonamento da plataforma é **avisar primeiro, banir apenas se o usuário reincidir**. A ferramenta Warn é o que torna essa política acionável: ela dá ao usuário uma chance de corrigir o rumo, e o registro do aviso é o que um agente futuro encontra quando procura na memória antes de considerar um banimento.

A ferramenta também remove duplicatas: se o agente já emitiu um aviso para o mesmo usuário sobre o mesmo comentário, um segundo aviso não faz nada. Assim, um LLM que entra em loop ou dispara novamente sobre o mesmo comentário não pode spammer o usuário com múltiplos avisos.

### What goes in the warning

Uma mensagem curta (limitada a 1000 caracteres) mostrada ao usuário como DM. Avisos eficazes são:

- **Específicos** - "Ataques pessoais a usuários identificados não são permitidos nesta comunidade" é melhor do que "seu comentário foi sinalizado."
- **Curtos** - no máximo algumas frases.
- **Acionáveis** - diga ao usuário o que mudar. "Por favor, edite seu comentário para remover o usuário identificado, ou ele será removido."

Você não escreve a mensagem você mesmo; o agente o faz, com base no [prompt inicial](#personality-prompt) e nas [diretrizes da comunidade](#community-guidelines). Seu trabalho é escrever um prompt que produza bons avisos.

### When to allow it

Para qualquer agente de estilo moderação. O modelo Moderator o habilita por padrão.

### Approvals

Menos frequentemente sujeito a aprovação do que [Ban user](#tool-ban-user). Vale a pena exigir aprovação durante as primeiras semanas de vida de um agente para que você possa identificar avisos ruins antes que eles sejam enviados, mas a maioria dos operadores remove essa exigência assim que o agente produz resultados confiáveis.

### See also

- [Ban user](#tool-ban-user) - the next step up in escalation.
- [Agent Memory System](#agent-memory-system) - where warning records live.