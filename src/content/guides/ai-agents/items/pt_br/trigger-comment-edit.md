---
Aciona o agente quando um comentário é editado.

### Contexto que o agente recebe

- O comentário em sua forma atual (pós-edição).
- O **texto anterior do comentário** como um bloco cercado separado (`PREVIOUS_TEXT`). Isso é exclusivo do gatilho de edição - o agente pode comparar antes/depois.
- Histórico opcional de thread/usuário/contexto da página conforme configurado.

### Observações

- O gatilho é acionado para qualquer edição bem-sucedida, incluindo edições realizadas por moderadores em nome de um usuário.
- Agentes não têm ferramenta de editar comentário disponível; agentes não podem editar comentários de forma alguma.
- O texto anterior do comentário é cercado como entrada não confiável. O prompt do sistema da plataforma lembra o modelo de não seguir instruções vindas de dentro de cercas - isso é importante aqui, pois um usuário mal-intencionado poderia editar um comentário para inserir um payload de "ignore your previous instructions" direcionado a qualquer agente que acompanhe eventos de edição.

### Usos comuns

- **Detectar conteúdo reintroduzido** - um usuário edita um comentário que estava limpo para inserir spam depois que o moderador já se afastou.
- **Rastrear edições menores** - se sua comunidade trata edições como eventos separados por qualquer motivo de auditoria.

### Observação sobre custos

Edit triggers see two copies of the comment text (the new version in the standard COMMENT block, the old version in the `PREVIOUS_TEXT` block). For long comments this roughly doubles the token cost of the run vs. a `COMMENT_ADD` trigger - keep that in mind when budgeting.

---