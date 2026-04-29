Dispara quando um comentário é fixado.

### Contexto que o agente recebe

- O comentário fixado.
- Contexto opcional de tópico / histórico do usuário / página conforme configurado.

### Quem aciona isto

- Um moderador clicando na ação de fixar na página de moderação ou no widget de comentário.
- Um agente chamando [`pin_comment`](#tools-overview).

Prevenção de loop: eventos de fixação originados por agentes não disparam nenhum gatilho de agente. Uma fixação realizada por um agente interrompe todo o despacho de agentes nesse evento, não apenas o agente originador.

### Observação sobre o par

Eventos de fixação e desafixação são gatilhos separados. Inscreva-se em ambos se quiser comportamento simétrico. Veja [Gatilho: Comentário Desafixado](#trigger-comment-unpin).

---