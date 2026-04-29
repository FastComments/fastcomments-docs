Dispara quando um usuário publica seu primeiro comentário neste site (seu tenant). Isso ocorre **uma vez por usuário** - comentários subsequentes do mesmo usuário não o disparam novamente.

### Contexto que o agente recebe

- O novo comentário.
- Contexto opcional de thread / histórico do usuário / página conforme configurado.

Quando o contexto de histórico do usuário está ativado, a lista de comentários recentes do usuário estará, obviamente, vazia (ou conterá apenas este), mas o fator de confiança e a idade da conta são preenchidos.

### Observações

- "First comment on this site" está limitado ao **tenant**, não ao site inteiro do FastComments. Um usuário com comentários em outros sites do FastComments ainda dispara este gatilho na primeira vez que publica no seu.
- O gatilho só dispara para usuários com um userId. Comentários anônimos não verificados sem um userId estável não o disparam.
- O gatilho dispara quando o comentário é aprovado/visível (não no momento da postagem inicial). Edições ou ações de moderador no primeiro comentário não o disparam novamente.

### Usos comuns

- **Saudação de boas-vindas** - o [Welcome Greeter template](#template-welcome-greeter) é baseado neste gatilho.
- **Onboarding** - envie um [aviso por DM](#tool-warn-user) (usado aqui como um lembrete amigável em vez de uma advertência) apontando o usuário para as diretrizes da comunidade.
- **Notificação para revisor** - se você quiser que um humano veja o primeiro post de cada novo comentarista, [`mark_comment_reviewed`](#tools-overview) pode marcá-los para revisão.

---