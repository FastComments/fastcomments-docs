Dispara quando um moderador aprova um comentário.

### Contexto que o agente recebe

- O comentário recém-aprovado.
- O **ID do usuário que acionou** - o moderador que aprovou.
- Contexto opcional do thread/histórico do usuário/página conforme configurado.

### Quem aciona isto

Uma ação de moderador humano.

### Observações

- Um comentário "aprovado" é um comentário **visível** na terminologia do FastComments. Veja [Como funcionam as aprovações](/guide-moderation.html#moderation-approvals) no guia de moderação para a distinção entre aprovado/não aprovado e revisado/não revisado.
- O gatilho dispara em aprovação **transições**: um comentário que passa de não aprovado para aprovado o dispara; um comentário que já estava aprovado e é salvo novamente não dispara.
- Para tenants onde os comentários default para auto-aprovação, esse gatilho dispara apenas quando um moderador reaprova explicitamente um comentário previamente oculto.

### Usos comuns

- **Boas-vindas / engajamento** - um agente pode responder a usuários que comentam pela primeira vez no momento em que um moderador os aprova, em vez de no momento da postagem.
- **Coordenação entre agentes** - se um agente separado havia marcado o comentário para revisão, a aprovação é o sinal de que a revisão humana terminou.
- **Trilha de auditoria** via [Webhooks](#webhooks-overview).