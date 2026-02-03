FastComments automaticamente acompanha eventos detalhados para cada comentário para fornecer transparência nas decisões de moderação e ações do sistema. Esses registros ajudam você a entender por que um comentário foi aprovado, sinalizado como spam ou teve seu status alterado.

## Acessando os registros de comentários

Para visualizar os registros de um comentário específico:

1. Navegue até a página **Moderate Comments** no painel do FastComments
2. Encontre o comentário que você deseja inspecionar
3. Clique no botão **View Logs** (ícone de relógio) na barra de ações do comentário
4. Um diálogo aparecerá exibindo o histórico completo de eventos para esse comentário

Cada entrada de registro exibe:
- **When** - O carimbo de data/hora do evento
- **Who** - O usuário ou sistema que acionou o evento (quando aplicável)
- **What** - O tipo de ação ou evento
- **Details** - Contexto adicional, como valores antes/depois, nomes de mecanismos ou dados relacionados

## Eventos de registro de comentário

Cada comentário mantém um registro dos eventos que ocorrem durante seu ciclo de vida. Abaixo estão os tipos de eventos que são rastreados:

### Eventos de anonimização
- **Anonymized** - O conteúdo do comentário foi apagado e o usuário marcado como excluído
- **RestoredFromAnonymized** - O comentário foi restaurado do estado anonimizado

### Eventos de aprovação
- **ApprovedDueToPastComment** - Comentário aprovado porque o usuário já tem comentários aprovados anteriormente (inclui referência ao comentário passado)
- **ApprovedIsAdmin** - Comentário aprovado porque o usuário é um administrador
- **NotApprovedRequiresApproval** - Comentário requer aprovação manual
- **NotApprovedLowTrustFactor** - Comentário não aprovado devido a baixo fator de confiança do usuário (inclui o valor do fator de confiança)

### Eventos de aprovação em comentários de perfil

Esses eventos se aplicam especificamente a comentários em perfis de usuário:

- **ApprovedProfileAutoApproveAll** - Comentário de perfil autoaprovado porque o dono do perfil habilitou autoaprovação para todos os comentários
- **ApprovedProfileTrusted** - Comentário de perfil aprovado porque o comentarista é confiável (inclui referência ao comentário que estabeleceu a confiança)
- **NotApprovedProfileManualApproveAll** - Comentário de perfil requer aprovação manual porque o dono do perfil habilitou aprovação manual
- **NotApprovedProfileNotTrusted** - Comentário de perfil não aprovado porque o comentarista não é confiável
- **NotApprovedProfileNewUser** - Comentário de perfil não aprovado porque o comentarista é um usuário novo

### Eventos de detecção de spam
- **IsSpam** - Comentário sinalizado como spam pelo mecanismo de detecção (inclui qual mecanismo tomou a decisão)
- **IsSpamDueToBadWords** - Comentário sinalizado como spam devido ao filtro de palavrões
- **IsSpamFromLLM** - Comentário sinalizado como spam pelo mecanismo de IA/LLM (inclui nome do mecanismo, resposta e contagem de tokens)
- **IsSpamRepeatComment** - Comentário sinalizado como spam por ser repetitivo (inclui qual mecanismo detectou)
- **NotSpamIsOnlyImage** - Comentário não sinalizado como spam porque contém apenas imagens
- **NotSpamIsOnlyReacts** - Comentário não sinalizado como spam porque contém apenas reações
- **NotSpamNoLinkOrMention** - Comentário não sinalizado como spam por não conter links ou menções suspeitas
- **NotSpamPerfectTrustFactor** - Comentário não sinalizado como spam devido ao alto fator de confiança do usuário
- **NotSpamTooShort** - Comentário não sinalizado como spam porque é curto demais para análise
- **NotSpamSkipped** - Verificação de spam foi pulada
- **NotSpamFromEngine** - Comentário determinado como não spam pelo mecanismo de detecção (inclui nome do mecanismo e fator de confiança)

### Eventos de palavrões/profanação
- **BadWordsCheckFailed** - A verificação do filtro de palavrões encontrou um erro
- **BadWordsFoundBadPhrase** - O filtro de palavrões detectou uma frase inadequada (inclui a frase)
- **BadWordsFoundBadWord** - O filtro de palavrões detectou uma palavra inadequada (inclui a palavra)
- **BadWordsNoDefinitionForLocale** - Não há definições de palavrões disponíveis para o idioma do comentário (inclui a localidade)

### Eventos de verificação do usuário
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - Comentário requer verificação, mas o usuário não está em uma sessão verificada
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - Comentário requer verificação, mas o usuário ainda não foi verificado
- **InVerifiedSession** - Usuário que postou o comentário está em uma sessão verificada
- **SentVerificationEmailNoSession** - E-mail de verificação enviado para usuário não verificado
- **SentWelcomeEmail** - E-mail de boas-vindas enviado ao novo usuário

### Eventos de confiança e segurança
- **TrustFactorChanged** - O fator de confiança do usuário foi modificado (inclui valores antes e depois)
- **SpamFilterDisabledBecauseAdmin** - Filtragem de spam ignorada para usuário administrador
- **TenantSpamFilterDisabled** - Filtragem de spam desabilitada para todo o tenant
- **RepeatCommentCheckIgnored** - Verificação de comentário repetido foi ignorada (inclui o motivo)
- **UserIsAdmin** - Usuário identificado como administrador
- **UserIsAdminParentTenant** - Usuário identificado como administrador do tenant pai
- **UserIsAdminViaSSO** - Usuário identificado como administrador via SSO
- **UserIsMod** - Usuário identificado como moderador

### Alterações de status do comentário

Eventos de alteração de status incluem valores antes e depois, além do usuário que realizou a alteração:

- **ExpireStatusChanged** - O status de expiração do comentário foi modificado
- **ReviewStatusChanged** - O status de revisão do comentário foi alterado
- **SpamStatusChanged** - O status de spam do comentário foi atualizado
- **ApproveStatusChanged** - O status de aprovação do comentário foi alterado
- **TextChanged** - O texto do comentário foi editado (inclui texto antes e depois)
- **VotesChanged** - As contagens de votos do comentário foram atualizadas (inclui detalhamento dos votos)
- **Flagged** - O comentário foi sinalizado por usuários
- **UnFlagged** - Sinais/flags do comentário foram removidos

### Ações de moderação
- **Pinned** - Comentário foi fixado pelo moderador (inclui quem fixou)
- **UnPinned** - Comentário foi desfixado pelo moderador (inclui quem desfiz)

### Eventos de notificação
- **CreatedNotifications** - Notificações foram criadas para o comentário (inclui contagem de notificações)
- **NotificationCreateFailure** - Falha ao criar notificações
- **BadgeAwarded** - Insígnia do usuário foi concedida pelo comentário (inclui nome da insígnia)

### Eventos de publicação
- **PublishedLive** - Comentário foi publicado para assinantes ao vivo (inclui contagem de assinantes)

### Eventos de integração
- **WebhookSynced** - Comentário foi sincronizado via webhook

### Eventos de regra de spam
- **SpamRuleMatch** - Comentário correspondeu a uma regra de spam personalizada (inclui detalhes da regra)

### Eventos de localização
- **LocaleDetectedFromText** - Localidade de idioma foi detectada automaticamente a partir do texto do comentário (inclui idioma e localidade detectados)

## Casos de uso para os registros de comentários

Os registros de comentários são gerados automaticamente e armazenados com cada comentário. Eles fornecem insights valiosos para:

- **Entender decisões de moderação** - Veja exatamente por que um comentário foi aprovado, mantido para revisão ou marcado como spam
- **Depurar problemas de aprovação/spam** - Rastreie a lógica de decisão quando os comentários não estão se comportando como esperado
- **Rastrear padrões de comportamento de usuários** - Monitore mudanças no fator de confiança e status de verificação
- **Auditar ações de moderadores** - Revise quais ações os moderadores tomaram em comentários específicos
- **Investigar a eficácia do filtro de spam** - Veja quais mecanismos de detecção estão identificando spam e quais não estão
- **Solução de problemas em integrações** - Verifique sincronizações de webhooks e entrega de notificações

Esses registros ajudam a manter a transparência no processo de moderação e auxiliam no ajuste fino do comportamento do seu sistema de comentários.