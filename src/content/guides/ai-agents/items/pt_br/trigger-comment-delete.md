É acionado quando um comentário é excluído.

### Contexto que o agente recebe

- O comentário que acabou de ser excluído - texto, autor, página.
- Contexto opcional de thread / histórico do usuário / da página, conforme configurado.

### Observações

- É acionado tanto para **soft deletes** (onde o comentário é ocultado mas mantido para auditoria) quanto para **hard deletes** (onde o comentário é completamente removido). O manipulador do gatilho resolve o comentário a partir do pipeline de exclusão em cascata; o que o agente vê é o último estado conhecido.
- Uma vez que um comentário é totalmente excluído, ferramentas que o têm como alvo (`pin_comment`, `mark_comment_spam`, etc.) nesse ID de comentário falharão.

### Usos comuns

- **Encaminhamento de auditoria via [Webhooks](#webhooks-overview)** - emite um evento `trigger.succeeded` para que um sistema externo registre o que foi excluído.
- **Gravações de memória** - faça com que o agente registre uma [nota de memória](#tools-overview) sobre um padrão de exclusão (o comentário excluído foi o terceiro do usuário em 24 horas, etc.).
- **Efeitos entre threads** - perceber quando uma exclusão altera a estrutura de um thread que o agente já resumiu anteriormente, e considerar se deve resumir novamente.

### Observação sobre custo de operação

Se você tiver um site com alto volume de exclusões (moderação humana intensa), esse gatilho pode acionar com frequência.

---