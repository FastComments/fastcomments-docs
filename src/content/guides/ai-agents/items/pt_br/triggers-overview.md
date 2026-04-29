Um **gatilho** é um evento que acorda um agente. Cada agente pode ter um ou mais gatilhos definidos.

### A lista completa

| Trigger | When it fires |
|---|---|
| [Comentário Adicionado](#trigger-comment-add) | Um novo comentário é publicado. |
| [Comentário Editado](#trigger-comment-edit) | Um comentário é editado. O texto anterior é incluído no contexto do agente. |
| [Comentário Excluído](#trigger-comment-delete) | Um comentário é excluído. |
| [Comentário Fixado](#trigger-comment-pin) | Um comentário é fixado (por qualquer pessoa, incluindo um moderador ou outro agente). |
| [Comentário Desfixado](#trigger-comment-unpin) | Um comentário é desfixado. |
| [Comentário Bloqueado](#trigger-comment-lock) | Um comentário é bloqueado (nenhuma resposta adicional permitida). |
| [Comentário Desbloqueado](#trigger-comment-unlock) | Um comentário é desbloqueado. |
| [Comentário Ultrapassa o Limite de Votos](#trigger-comment-vote-threshold) | O total líquido de votos de um comentário atinge o limite configurado. |
| [Comentário Atinge o Limite de Sinalizações](#trigger-comment-flag-threshold) | A contagem de sinalizações de um comentário atinge exatamente o limite configurado. |
| [Usuário Publica o Primeiro Comentário](#trigger-new-user-first-comment) | Um usuário publica seu primeiro comentário neste site. |
| [Comentário Marcado Automaticamente como Spam](#trigger-comment-auto-spammed) | Um comentário é auto-marcado como spam pelo mecanismo anti-spam. |
| [Moderador Revê Comentário](#trigger-moderator-reviewed) | Um moderador marca um comentário como revisado. |
| [Moderador Aprova Comentário](#trigger-moderator-approved) | Um moderador aprova um comentário. |
| [Moderador Marca como Spam](#trigger-moderator-spammed) | Um moderador marca um comentário como spam. |
| [Moderador Concede Distintivo](#trigger-moderator-awarded-badge) | Um moderador concede um distintivo a um usuário. |

### Múltiplos gatilhos por agente

Um agente pode subscrever qualquer combinação de gatilhos - o [Modelo de Moderador](#template-moderator) subscreve tanto `COMMENT_ADD` quanto `COMMENT_FLAG_THRESHOLD`, por exemplo. Cada evento dispara o agente uma vez com o contexto desse evento.

### O que impede um agente de ser acionado

Um evento de gatilho inscrito **não** aciona o agente se qualquer uma das seguintes condições ocorrer:

- O [status](#status-states) do agente está **Desativado**.
- O [escopo de URL ou localidade](#scope-url-locale) do agente não corresponde ao comentário que disparou.
- O [orçamento diário, mensal ou de limite de taxa](#budgets-overview) do agente está esgotado - o gatilho é registrado como **Descartado** com um motivo. Veja [Motivos de Descarte](#drop-reasons).
- A concorrência para este agente está saturada (limitada por agente).
- O tenant do agente tem cobrança inválida.
- A ação que acionou foi feita por um bot ou outro agente (prevenção de loop).
- O gatilho foi para um comentário que já foi processado por este agente dentro da janela de desduplicação.

Quando um gatilho inscrito dispara com sucesso, o [Histórico de Execuções](#run-history) do agente mostra uma linha com o status **Iniciado** que transita para **Sucesso** ou **Erro** quando a execução é concluída.

### Limites de votos e sinalizações

Dois gatilhos - **Comentário Ultrapassa o Limite de Votos** e **Comentário Atinge o Limite de Sinalizações** - exigem um valor numérico no formulário de edição. O gatilho dispara no momento em que a contagem cruza o valor configurado (especificamente, o gatilho de limite de sinalizações dispara quando `flagCount === flagThreshold`, então escolher 1 significa "disparar na primeira sinalização", e escolher 5 significa "disparar quando a quinta sinalização chegar").

### Gatilhos diferidos

Qualquer gatilho pode ser adiado para que o agente execute mais tarde, por exemplo após votos/sinalizações/respostas terem tempo para se estabilizar. Veja [Gatilhos Diferidos](#trigger-deferred-delay).

### Prevenção de loop

Para evitar loops infinitos, comentários **escritos por um agente** carregam um `botId`. Gatilhos de novo comentário ignoram comentários com um `botId`.

O efeito líquido: agentes podem agir em resposta a ações *humanas* em seu tenant, mas ações originadas por agentes nunca disparam quaisquer gatilhos de agente. Isso se aplica a todos os tipos de gatilho.

### REPLAY: o gatilho interno

Existe também um tipo de gatilho interno `REPLAY` usado pela funcionalidade [Execuções de Teste (Replays)](#test-runs-replays). Você não pode selecioná-lo no formulário de edição - ele existe para que as execuções de replay sejam marcadas distintamente no histórico de execuções e excluídas das visualizações de execuções ao vivo.