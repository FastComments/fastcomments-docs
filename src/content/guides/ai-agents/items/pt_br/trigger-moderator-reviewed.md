---
Dispara quando um moderador marca um comentário como revisado.

### Contexto que o agente recebe

- O comentário.
- O **ID do usuário que acionou** - o moderador que revisou.
- Histórico opcional de tópico / do usuário / contexto da página conforme configurado.

### Quem dispara isto

Uma ação de um moderador humano na página de moderação, no widget de comentários ou via API.

### Usos comuns

- **Encaminhamento de auditoria** via [Webhooks](#webhooks-overview).
- **Gravações de memória** - registre uma nota de memória informando que este comentário foi revisado por um humano para que outros agentes não o processem em duplicidade.

### Observações

- "Revisado" é um dos estados da fila de moderação rastreados separadamente de "aprovado" e "spam". Um comentário pode ser aprovado-e-revisado, aprovado-mas-não-revisado, etc. Veja [Como Funcionam as Aprovações](/guide-moderation.html#moderation-approvals) no guia de moderação.
- Este gatilho possui alta frequência em locatários com muitos moderadores. Inscreva-se seletivamente e planeje o orçamento de acordo.

---