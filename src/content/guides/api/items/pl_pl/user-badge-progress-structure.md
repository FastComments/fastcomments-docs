`UserBadgeProgress` to obiekt, który reprezentuje postęp użytkownika w zdobywaniu różnych odznak w systemie FastComments.

To śledzenie pomaga określić, kiedy użytkownicy powinni otrzymać automatyczne odznaki na podstawie ich aktywności i uczestnictwa w Twojej społeczności.

Struktura obiektu `UserBadgeProgress` jest następująca:

[inline-code-attrs-start title = 'Struktura UserBadgeProgress'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** Unikalny identyfikator tego rekordu postępu */
    id: string
    /** ID tenanta, do którego należy ten rekord postępu */
    tenantId: string
    /** ID użytkownika, którego dotyczy ten rekord postępu */
    userId: string
    /** ID pierwszego komentarza użytkownika w systemie */
    firstCommentId?: string
    /** Data pierwszego komentarza użytkownika (milisekundy od epoki Unix) */
    firstCommentDate?: number
    /** Automatycznie obliczany współczynnik zaufania na podstawie aktywności użytkownika */
    autoTrustFactor?: number
    /** Współczynnik zaufania ustawiony ręcznie przez administratorów */
    manualTrustFactor?: number
    /** Szczegółowy obiekt postępu z różnymi miarami, klucze odpowiadają enumeracji BadgeType */
    progress: {
        /** 0: CommentCount - Liczba komentarzy, które użytkownik napisał */
        '0'?: number
        /** 1: CommentUpVotes - Liczba upvote'ów, które użytkownik otrzymał */
        '1'?: number
        /** 2: CommentReplies - Liczba odpowiedzi, które użytkownik napisał */
        '2'?: number
        /** 3: CommentsPinned - Liczba przypiętych komentarzy, które posiada użytkownik */
        '3'?: number
        /** 4: Veteran - Wiek konta użytkownika */
        '4'?: number
        /** 5: NightOwl - Liczba razy, gdy użytkownik publikował w godzinach nocnych */
        '5'?: number
        /** 6: FastReplyTime - Średni czas odpowiedzi w milisekundach */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - W przypadku odznak moderatora, liczba usuniętych komentarzy */
        '7'?: number
        /** 8: ModeratorCommentsApproved - W przypadku odznak moderatora, liczba zatwierdzonych komentarzy */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - W przypadku odznak moderatora, liczba niezatwierdzonych komentarzy */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - W przypadku odznak moderatora, liczba przejrzanych komentarzy */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - W przypadku odznak moderatora, liczba komentarzy oznaczonych jako spam */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - W przypadku odznak moderatora, liczba komentarzy oznaczonych jako nie-spam */
        '12'?: number
        /** 13: RepliedToSpecificPage - Dla każdej strony, liczba odpowiedzi */
        '13'?: Record<string, number>
    }
}
[inline-code-end]