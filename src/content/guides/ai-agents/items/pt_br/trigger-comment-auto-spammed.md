Dispara quando um comentário é marcado automaticamente como spam pelo mecanismo de spam interno do FastComments - **não** por um moderador e nem por outro agente.

### Contexto que o agente recebe

- O comentário marcado automaticamente como spam.
- Histórico opcional do tópico/usuário/contexto da página, conforme configurado.

### Quem aciona isso

O pipeline de spam da plataforma. Veja [Detecção de Spam](/guide-moderation.html#spam-detection) no guia de moderação para mais detalhes.

### Usos comuns

- **Moderação de segunda análise** - o mecanismo de spam tem alta taxa de detecção (recall) mas precisão imperfeita; um agente treinado no estilo específico da sua comunidade pode detectar falsos positivos. O agente pode chamar para desmarcar um comentário classificado incorretamente.
- **Desbanimento automatizado** - se seu tenant bane agressivamente novas contas por spam, um agente nesse gatilho pode revisar e liberar falsos positivos óbvios antes que um humano os veja.

### Observações

- O gatilho **não** dispara para spam marcado por um moderador (use [Gatilho: Spam Marcado por Moderador](#trigger-moderator-spammed)) nem para spam marcado por outro agente.
- Um comentário que é marcado automaticamente como spam e depois é marcado como Não Spam por um moderador não dispara o gatilho novamente.
- Assinar este gatilho é mais útil em tenants onde o modo de auto-spam do mecanismo está habilitado em Configurações de Moderação. Caso contrário o gatilho não disparará.