Dispara quando um moderador concede um distintivo a um usuário.

### Contexto que o agente recebe

- O **ID do distintivo** concedido.
- O **ID do usuário que acionou** - o moderador que concedeu o distintivo.
- Contexto opcional de thread / histórico do usuário / página conforme configurado.

O local de disparo **não** inclui um `commentId` no payload do gatilho, mesmo se o distintivo foi concedido em relação a um comentário específico.

### Quem dispara isso

Uma ação de um moderador humano.

### Observações

- Somente o ID do distintivo é incluído; o agente não recebe os metadados do distintivo (nome, imagem). Se o agente precisar inferir *qual* distintivo foi concedido, incorpore esse contexto no [prompt inicial](#personality-prompt) ou nas [diretrizes da comunidade](#community-guidelines).
- O gatilho dispara uma vez por concessão de distintivo, não por usuário. Conceder o mesmo distintivo a um usuário duas vezes dispara o evento duas vezes (cada concessão é um evento distinto).

### Usos comuns

- **Reconhecimento recíproco** - um agente pode postar uma resposta "obrigado pela ótima contribuição" quando um distintivo específico é concedido.
- **Fluxo de reconhecimento externo** via [Webhooks](#webhooks-overview) - espelhe as concessões de distintivos no seu próprio sistema de engajamento de usuários.
- **Registro de memória** - notas como "este usuário é um colaborador reconhecido" para que futuros agentes de moderação considerem isso em suas decisões.

---