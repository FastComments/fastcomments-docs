Dispara quando um comentário é bloqueado.

### Contexto que o agente recebe

- O comentário bloqueado.
- Histórico opcional de thread / de usuário / contexto da página conforme configurado.

### Quem dispara este evento

- Um moderador usando a ação de bloqueio na página de moderação ou no widget de comentários.

### Usos comuns

- **Notificar revisores** - um evento de bloqueio frequentemente segue um tópico acalorado; um webhook para seu canal de moderação no Slack pode permitir que humanos assumam o restante.
- **Aplicação do período de espera** - agende um [acionador adiado](#trigger-deferred-delay) em um agente separado que, 24 horas após o bloqueio, avalie se deve desbloquear.

### Par

Veja [Acionador: Comentário Desbloqueado](#trigger-comment-unlock) para o acionador espelho.

---