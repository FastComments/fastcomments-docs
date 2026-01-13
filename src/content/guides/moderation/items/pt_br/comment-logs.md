FastComments automaticamente registra eventos detalhados para cada comentário para fornecer transparência nas decisões de moderação e nas ações do sistema. Esses registros ajudam você a entender por que um comentário foi aprovado, marcado como spam ou teve seu status alterado.

Você pode visualizar os registros de comentários para comentários individuais no painel Moderar Comentários selecionando um comentário específico.

## Eventos do registro de comentários

Cada comentário mantém um registro de eventos que ocorrem durante seu ciclo de vida. Abaixo estão os tipos de eventos que são monitorados:

### Eventos de anonimização
- **Anonymized** - O conteúdo do comentário foi limpo e o usuário marcado como excluído

### Eventos de aprovação
- **ApprovedDueToPastComment** - Comentário aprovado porque o usuário teve comentários aprovados anteriormente
- **ApprovedIsAdmin** - Comentário aprovado porque o usuário é um administrador
- **NotApprovedRequiresApproval** - Comentário requer aprovação manual

### Eventos de detecção de spam
- **IsSpam** - Comentário marcado como spam pelo mecanismo de detecção
- **IsSpamDueToBadWords** - Comentário marcado como spam devido ao filtro de palavrões
- **IsSpamFromLLM** - Comentário marcado como spam pelo mecanismo de IA/LLM
- **IsSpamRepeatComment** - Comentário marcado como spam por ser repetitivo
- **NotSpamIsOnlyImage** - Comentário não marcado como spam porque contém apenas imagens
- **NotSpamIsOnlyReacts** - Comentário não marcado como spam porque contém apenas reações
- **NotSpamNoLinkOrMention** - Comentário não marcado como spam devido à ausência de links ou menções suspeitas
- **NotSpamPerfectTrustFactor** - Comentário não marcado como spam devido à alta confiança do usuário
- **NotSpamTooShort** - Comentário não marcado como spam por ser curto demais para análise
- **NotSpamSkipped** - Verificação de spam foi ignorada
- **NotSpamFromEngine** - Comentário determinado como não spam pelo mecanismo de detecção

### Eventos de palavrões/profanidade
- **BadWordsCheckFailed** - A verificação do filtro de palavrões encontrou um erro
- **BadWordsFoundBadPhrase** - O filtro de palavrões detectou uma frase inadequada
- **BadWordsFoundBadWord** - O filtro de palavrões detectou uma palavra inadequada
- **BadWordsNoDefinitionForLocale** - Não há definições de palavrões disponíveis para o idioma do comentário

### Eventos de verificação do usuário
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Comentário requer verificação, mas o usuário não está em uma sessão verificada
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Comentário requer verificação, mas o usuário ainda não foi verificado
- **InVerifiedSession** - Usuário que postou o comentário está em uma sessão verificada
- **SentVerificationEmailNoSession** - E-mail de verificação enviado para usuário não verificado
- **SentWelcomeEmail** - E-mail de boas-vindas enviado para novo usuário

### Eventos de confiança e segurança
- **TrustFactorChanged** - O fator de confiança do usuário foi modificado
- **SpamFilterDisabledBecauseAdmin** - Filtragem de spam contornada por usuário administrador
- **TenantSpamFilterDisabled** - Filtragem de spam desativada para todo o tenant
- **RepeatCommentCheckIgnored** - Verificação de comentário repetido foi ignorada
- **UserIsAdmin** - Usuário identificado como administrador
- **UserIsAdminParentTenant** - Usuário identificado como administrador do tenant pai
- **UserIsAdminViaSSO** - Usuário identificado como administrador via SSO
- **UserIsMod** - Usuário identificado como moderador

### Alterações no status do comentário
- **ExpireStatusChanged** - O status de expiração do comentário foi modificado
- **ReviewStatusChanged** - O status de revisão do comentário foi alterado
- **SpamStatusChanged** - O status de spam do comentário foi atualizado
- **ApproveStatusChanged** - O status de aprovação do comentário foi alterado
- **TextChanged** - O conteúdo de texto do comentário foi editado
- **VotesChanged** - As contagens de votos do comentário foram atualizadas
- **Flagged** - Comentário foi sinalizado por usuários
- **UnFlagged** - Sinalizações do comentário foram removidas

### Ações de moderação
- **Pinned** - Comentário foi fixado pelo moderador
- **UnPinned** - Comentário foi desafixado pelo moderador
- **RestoredFromAnonymized** - Comentário foi restaurado do estado anonimizado

### Eventos de notificação
- **CreatedNotifications** - Notificações foram criadas para o comentário
- **NotificationCreateFailure** - Falha ao criar notificações
- **BadgeAwarded** - Emblema do usuário foi concedido pelo comentário

### Eventos de publicação
- **PublishedLive** - Comentário foi publicado para assinantes ao vivo

### Eventos de integração
- **WebhookSynced** - Comentário foi sincronizado via webhook

### Eventos de regra de spam
- **SpamRuleMatch** - Comentário correspondeu a uma regra de spam personalizada

## Acessando os registros de comentários

Os registros de comentários são gerados automaticamente e armazenados com cada comentário. Eles fornecem informações valiosas para:

- Entender decisões de moderação
- Depurar problemas de aprovação/spam
- Acompanhar padrões de comportamento do usuário
- Auditar ações do sistema

Esses registros ajudam a manter a transparência no processo de moderação e auxiliam no ajuste fino do comportamento do seu sistema de comentários.