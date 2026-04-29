Dispara quando um moderador marca um comentário como spam.

### Contexto que o agente recebe

- O comentário, com a flag pós-ação `Is Spam`.
- O **ID do usuário acionador** - o moderador que atuou.
- Thread / histórico do usuário / contexto de página opcionais, conforme configurado.

### Quem dispara isto

Uma ação de moderador humano. Marcações de spam originadas por agente (via [`mark_comment_spam`](#tools-overview)) **não** disparam este gatilho.

### Usos comuns

- **Registro de memória** - fazer com que um agente salve uma nota de memória sobre o usuário marcado como spam (por exemplo, "anteriormente marcado como spam por X por um moderador") para que agentes de moderação futuros tenham contexto.
- **Aplicação em nível de usuário** - a marcação de um comentário como spam por um moderador pode ser o sinal para que um agente também emita um aviso ou uma suspensão curta, sujeita à aprovação.
- **Espelhamento para sistema externo** via [Webhooks](#webhooks-overview).

---