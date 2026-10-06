FastComments rastreia automaticamente eventos detalhados para cada comentário, proporcionando transparência nas decisões de moderação e nas ações do sistema. Esses registros ajudam a entender por que um comentário foi aprovado, marcado como spam ou teve seu status alterado.

## Acessando Registros de Comentários

1. Navegue até a página **Moderate Comments** no seu painel do FastComments  
2. Encontre o comentário que deseja inspecionar  
3. Clique no botão **View Logs** (ícone de relógio) na barra de ações do comentário  
4. Um diálogo aparecerá mostrando o histórico completo de eventos para esse comentário  

Cada entrada de registro exibe:
- **When** - O carimbo de data/hora do evento  
- **Who** - O usuário ou sistema que disparou o evento (quando aplicável)  
- **What** - O tipo de ação ou evento  
- **Details** - Contexto adicional como valores antes/depois, nomes de mecanismos ou dados relacionados  

## Eventos de Registro de Comentários

Cada comentário mantém um registro de eventos que ocorrem durante seu ciclo de vida. Abaixo estão os tipos de eventos que são rastreados:

### Eventos de Anonimização
- **Anonymized** - O conteúdo do comentário foi apagado e o usuário marcado como excluído  
- **RestoredFromAnonymized** - O comentário foi restaurado do estado anonimizado  

### Eventos de Aprovação
- **ApprovedDueToPastComment** - Comentário aprovado porque o usuário já aprovou comentários anteriormente (inclui referência ao comentário passado)  
- **ApprovedIsAdmin** - Comentário aprovado porque o usuário é um administrador  
- **NotApprovedRequiresApproval** - Comentário requer aprovação manual  
- **NotApprovedLowTrustFactor** - Comentário não aprovado devido a baixo fator de confiança do usuário (inclui o valor do fator de confiança)  

### Eventos de Aprovação de Comentário de Perfil
Esses eventos se aplicam especificamente a comentários em perfis de usuário:
- **ApprovedProfileAutoApproveAll** - Comentário de perfil autoaprovado porque o proprietário do perfil habilitou autoaprovação para todos os comentários  
- **ApprovedProfileTrusted** - Comentário de perfil aprovado porque o comentarista é confiável (inclui referência ao comentário que estabeleceu a confiança)  
- **NotApprovedProfileManualApproveAll** - Comentário de perfil requer aprovação manual porque o proprietário do perfil habilitou aprovação manual  
- **NotApprovedProfileNotTrusted** - Comentário de perfil não aprovado porque o comentarista não é confiável  
- **NotApprovedProfileNewUser** - Comentário de perfil não aprovado porque o comentarista é um usuário novo  

### Eventos de Detecção de Spam
- **IsSpam** - Comentário marcado como spam pelo mecanismo de detecção (inclui qual mecanismo tomou a decisão)  
- **IsSpamDueToBadWords** - Comentário marcado como spam devido ao filtro de palavrões  
- **IsSpamFromLLM** - Comentário marcado como spam por motor de IA/LLM (inclui nome do motor, resposta e contagem de tokens)  
- **IsSpamRepeatComment** - Comentário marcado como spam por ser repetitivo (inclui qual motor detectou)  
- **NotSpamIsOnlyImage** - Comentário não marcado como spam porque contém apenas imagens  
- **NotSpamIsOnlyReacts** - Comentário não marcado como spam porque contém apenas reações  
- **NotSpamNoLinkOrMention** - Comentário não marcado como spam devido à ausência de links ou menções suspeitas  
- **NotSpamPerfectTrustFactor** - Comentário não marcado como spam devido à alta confiança do usuário  
- **NotSpamTooShort** - Comentário não marcado como spam porque é muito curto para analisar  
- **NotSpamSkipped** - Verificação de spam foi ignorada  
- **NotSpamFromEngine** - Comentário determinado como não spam pelo mecanismo de detecção (inclui nome do motor e fator de confiança)  

### Eventos de Palavrões/Profanidade
- **BadWordsCheckFailed** - Verificação do filtro de profanidade encontrou um erro  
- **BadWordsFoundBadPhrase** - Filtro de profanidade detectou frase inadequada (inclui a frase)  
- **BadWordsFoundBadWord** - Filtro de profanidade detectou palavra inadequada (inclui a palavra)  
- **BadWordsNoDefinitionForLocale** - Nenhuma definição de profanidade disponível para o idioma do comentário (inclui a localidade)  

### Eventos de Verificação de Usuário
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Comentário requer verificação, mas o usuário não está em sessão verificada  
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Comentário requer verificação, mas o usuário ainda não foi verificado  
- **InVerifiedSession** - Usuário que postou o comentário está em uma sessão verificada  
- **SentVerificationEmailNoSession** - E‑mail de verificação enviado ao usuário não verificado  
- **SentWelcomeEmail** - E‑mail de boas‑vindas enviado ao novo usuário  

### Eventos de Confiança e Segurança
- **TrustFactorChanged** - O fator de confiança do usuário foi modificado (inclui valores antes e depois)  
- **SpamFilterDisabledBecauseAdmin** - Filtragem de spam ignorada para usuário administrador  
- **TenantSpamFilterDisabled** - Filtragem de spam desativada para todo o locatário  
- **RepeatCommentCheckIgnored** - Verificação de comentário repetido foi ignorada (inclui o motivo)  
- **UserIsAdmin** - Usuário identificado como administrador  
- **UserIsAdminParentTenant** - Usuário identificado como administrador do locatário pai  
- **UserIsAdminViaSSO** - Usuário identificado como administrador via SSO  
- **UserIsMod** - Usuário identificado como moderador  

### Alterações de Status de Comentário
Eventos de alteração de status incluem valores antes e depois, além do usuário que fez a alteração:
- **ExpireStatusChanged** - O status de expiração do comentário foi modificado  
- **ReviewStatusChanged** - O status de revisão do comentário foi alterado  
- **SpamStatusChanged** - O status de spam do comentário foi atualizado  
- **ApproveStatusChanged** - O status de aprovação do comentário foi alterado  
- **TextChanged** - O conteúdo de texto do comentário foi editado (inclui texto antes e depois)  
- **VotesChanged** - As contagens de votos do comentário foram atualizadas (inclui detalhamento dos votos)  
- **Flagged** - Comentário foi sinalizado por usuários  
- **UnFlagged** - Sinalizações do comentário foram removidas  

### Ações de Moderação
- **Pinned** - Comentário foi fixado por moderador (inclui quem o fixou)  
- **UnPinned** - Comentário foi desafixado por moderador (inclui quem o desafixou)  

### Eventos de Notificação
- **CreatedNotifications** - Notificações foram criadas para o comentário (inclui contagem de notificações)  
- **NotificationCreateFailure** - Falha ao criar notificações  
- **BadgeAwarded** - Emblema de usuário foi concedido por comentário (inclui nome do emblema)  

### Eventos de Menção e Notificação de Resposta
Esses eventos nomeiam a pessoa que receberia o e‑mail ou notificação. Quando nada foi enviado, a coluna Detalhes indica o motivo.
- **MentionEmailSent** - Um usuário mencionado no comentário recebeu e‑mail  
- **MentionEmailSkipped** - Um usuário mencionado não recebeu e‑mail (inclui o motivo)  
- **MentionHeldForApproval** - O e‑mail de menção está aguardando até que o comentário seja aprovado  
- **MentionNotificationCreated** - Um usuário mencionado recebeu uma notificação no aplicativo  
- **MentionNotificationSkipped** - Um usuário mencionado não recebeu notificação no aplicativo (inclui o motivo)  
- **ReplyEmailSent** - O autor do comentário ao qual se respondeu recebeu e‑mail sobre esta resposta  
- **ReplyEmailSkipped** - O autor do comentário ao qual se respondeu não recebeu e‑mail (inclui o motivo)  
- **ReplyNotificationSkipped** - O autor do comentário ao qual se respondeu não recebeu notificação no aplicativo (inclui o motivo)  

Razões mostradas quando um e‑mail ou notificação não foi enviado:
- O usuário não existe mais, ou não tem endereço de e‑mail  
- O usuário desativou notificações por e‑mail, ou desativou notificações para esse tópico  
- Um dos dois usuários bloqueou o outro  
- Os usuários não estão em nenhum dos mesmos grupos SSO  
- O endereço de e‑mail do usuário está na lista de supressão após um bounce ou reclamação de spam (veja [Email Suppression Management](/guide-notifications.html#email-suppression-management))  
- O endereço de e‑mail do usuário é @example.com, que não pode receber e‑mail  
- O comentário foi marcado como spam, excluído, ou não aprovado dentro de 7 dias  
- O comentário ao qual se respondeu foi deixado anonimamente  
- O usuário respondeu ao próprio comentário  
- O usuário foi mencionado na resposta, então recebeu o e‑mail de menção em vez do e‑mail de resposta  
- O usuário já tinha uma notificação de resposta para o comentário  
- O envio falhou 5 vezes  

Se a entrega falhar ou atingir um limite de envio, o e‑mail é colocado em fila para nova tentativa e a entrada de registro indica isso.

### Eventos de Publicação
- **PublishedLive** - Comentário foi publicado para assinantes ao vivo (inclui contagem de assinantes)  

### Eventos de Integração
- **WebhookSynced** - Comentário foi sincronizado via webhook  

### Eventos de Regra de Spam
- **SpamRuleMatch** - Comentário correspondeu a uma regra de spam personalizada (inclui detalhes da regra)  

### Eventos de Localização
- **LocaleDetectedFromText** - Localidade de idioma foi detectada automaticamente a partir do texto do comentário (inclui idioma e localidade detectados)  

## Casos de Uso para Registros de Comentários

Registros de comentários são gerados automaticamente e armazenados com cada comentário. Eles fornecem insights valiosos para:
- **Understanding moderation decisions** - Veja exatamente por que um comentário foi aprovado, retido para revisão ou marcado como spam  
- **Debugging approval/spam issues** - Rastreie a lógica de decisão quando os comentários não se comportam como esperado  
- **Tracking user behavior patterns** - Monitore mudanças no fator de confiança e status de verificação  
- **Auditing moderator actions** - Revise quais ações os moderadores tomaram em comentários específicos  
- **Investigating spam filter effectiveness** - Veja quais mecanismos de detecção estão capturando spam e quais não  
- **Troubleshooting integrations** - Verifique sincronizações de webhook e entrega de notificações  

Esses registros ajudam a manter a transparência no processo de moderação e auxiliam no ajuste fino do comportamento do seu sistema de comentários.