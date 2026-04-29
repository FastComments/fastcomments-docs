Dispara o agente toda vez que um novo comentário é postado em uma página coberta pelo [escopo](#scope-url-locale) do agente.

### Contexto que o agente recebe

- O novo comentário na íntegra - texto, autor, votos, parent ID, page URL ID.
- Opcional: comentário pai e respostas anteriores no mesmo tópico, se [contexto do tópico](#context-options) estiver ativado.
- Opcional: fator de confiança do autor do comentário, idade da conta, histórico de banimentos e comentários recentes, se [contexto de histórico do usuário](#context-options) estiver ativado.
- Opcional: metadados da página, se [contexto da página](#context-options) estiver ativado.

### Observações

- O gatilho é acionado **depois** que o comentário foi persistido. O agente pode referir-se a ele diretamente em chamadas de ferramentas.
- Ele **não** é acionado para comentários escritos por outro agente no mesmo tenant.
- É acionado tanto para comentários verificados quanto não verificados. Se o seu tenant exigir aprovação de um moderador antes que um comentário fique visível (veja [Como Funciona a Aprovação](/guide-moderation.html#moderation-approvals) no guia de moderação), o gatilho é acionado quando o comentário é criado, não quando ele for aprovado posteriormente. O bot moderador pode ser instruído a aprovar comentários para você após análise.

### Usos comuns

- **Moderação** - verificar o comentário em relação às diretrizes da comunidade, marcar como spam ou advertir usuários de primeira vez.
- **Saudação de boas-vindas** - embora [Gatilho: Primeiro Comentário de Novo Usuário](#trigger-new-user-first-comment) geralmente seja mais adequado para saudações, já que é acionado uma vez por usuário.
- **Sumarização do tópico** - geralmente em conjunto com um [atraso do gatilho](#trigger-deferred-delay) para que o tópico se estabilize antes de o agente ser executado.

---