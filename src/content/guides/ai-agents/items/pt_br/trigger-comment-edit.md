Dispara o agente quando um comentário é editado.

### Contexto que o agente recebe

- O comentário em sua forma atual (após a edição).
- O **texto anterior do comentário** como um bloco delimitado separado (`PREVIOUS_TEXT`). Isso é exclusivo do gatilho de edição - o agente pode comparar antes/depois.
- Histórico opcional de thread / usuário / contexto da página, conforme configurado.

### Observações importantes

- O gatilho é disparado para qualquer edição bem-sucedida, incluindo edições realizadas por moderadores em nome de um usuário.
- Os agentes não têm nenhuma ferramenta de edição de comentários exposta a eles; agentes não podem editar comentários de forma alguma.
- O texto anterior do comentário é delimitado como entrada não confiável. O prompt de sistema da plataforma lembra o modelo de não seguir instruções de dentro dos delimitadores - isso importa aqui, porque um usuário mal-intencionado poderia editar um comentário para inserir um payload de "ignore suas instruções anteriores" direcionado a qualquer agente que esteja observando eventos de edição.

### Usos comuns

- **Detectar conteúdo lavado** - um usuário edita um comentário anteriormente limpo para inserir spam depois que o moderador já passou por ele.
- **Rastrear edições menores** - se sua comunidade trata edições como eventos separados por qualquer motivo de auditoria.

### Nota sobre custo

Os gatilhos de edição veem duas cópias do texto do comentário (a nova versão no bloco COMMENT padrão, a versão antiga no bloco PREVIOUS_TEXT). Para comentários longos, isso praticamente dobra o custo em tokens da execução em comparação com um gatilho `COMMENT_ADD` - tenha isso em mente ao fazer o orçamento.