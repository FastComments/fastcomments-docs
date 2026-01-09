`UserBadgeProgress` é um objeto que representa o progresso de um usuário na conquista de várias insígnias no sistema FastComments.

Esse acompanhamento ajuda a determinar quando os usuários devem receber insígnias automáticas com base em sua atividade e participação na sua comunidade.

A estrutura do objeto `UserBadgeProgress` é a seguinte:

[inline-code-attrs-start title = 'Estrutura UserBadgeProgress'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Identificador único para este registro de progresso */
    id: string
    /** ID do locatário ao qual este registro de progresso pertence */
    tenantId: string
    /** ID do usuário que este registro de progresso rastreia */
    userId: string
    /** ID do primeiro comentário do usuário no sistema */
    firstCommentId?: string
    /** Data do primeiro comentário do usuário (milissegundos desde epoch) */
    firstCommentDate?: number
    /** Fator de confiança calculado automaticamente com base na atividade do usuário */
    autoTrustFactor?: number
    /** Fator de confiança definido manualmente pelos administradores */
    manualTrustFactor?: number
    /** Objeto de progresso detalhado com várias métricas, chaves correspondem ao enum BadgeType */
    progress: {
        /** 0: CommentCount - Contagem de comentários que o usuário fez */
        '0'?: number
        /** 1: CommentUpVotes - Contagem de votos positivos que o usuário recebeu */
        '1'?: number
        /** 2: CommentReplies - Contagem de respostas que o usuário fez */
        '2'?: number
        /** 3: CommentsPinned - Contagem de comentários fixados que o usuário possui */
        '3'?: number
        /** 4: Veteran - Idade da conta do usuário */
        '4'?: number
        /** 5: NightOwl - Quantas vezes o usuário postou durante horários noturnos */
        '5'?: number
        /** 6: FastReplyTime - Tempo médio de resposta em milissegundos */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - Para insígnias de moderador, contagem de comentários excluídos */
        '7'?: number
        /** 8: ModeratorCommentsApproved - Para insígnias de moderador, contagem de comentários aprovados */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - Para insígnias de moderador, contagem de comentários não aprovados */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - Para insígnias de moderador, contagem de comentários revisados */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - Para insígnias de moderador, contagem de comentários marcados como spam */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - Para insígnias de moderador, contagem de comentários marcados como não spam */
        '12'?: number
        /** 13: RepliedToSpecificPage - Para cada página, contagem de respostas */
        '13'?: Record<string, number>
    }
}
[inline-code-end]
---