`UserBadge` jest obiektem, który reprezentuje odznakę przypisaną użytkownikowi w systemie FastComments.

Odznaki mogą być przydzielane użytkownikom automatycznie na podstawie ich aktywności (takiej jak liczba komentarzy, czas odpowiedzi, status weterana) lub ręcznie przez administratorów strony.

Struktura dla obiektu `UserBadge` jest następująca:

[inline-code-attrs-start title = 'Struktura UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Unikalny identyfikator przypisania tej odznaki użytkownikowi */
    id: string
    /** ID użytkownika, któremu przypisano tę odznakę */
    userId: string
    /** ID definicji odznaki z katalogu odznak najemcy */
    badgeId: string
    /** ID najemcy, który utworzył/przypisał tę odznakę */
    fromTenantId: string
    /** Kiedy ta odznaka została utworzona (milisekundy od epoki) */
    createdAt?: number
    /** Kiedy użytkownik otrzymał tę odznakę (milisekundy od epoki) */
    receivedAt?: number
    /** 
     * Typ odznaki: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** Dla odznak opartych na progach, wartość progu */
    threshold?: number
    /** Nazwa/etykieta odznaki */
    name?: string
    /** Szczegółowy opis odznaki */
    description?: string
    /** Tekst wyświetlany na odznace */
    displayLabel?: string
    /** URL do obrazu wyświetlanego na odznace */
    displaySrc?: string
    /** Kolor tła odznaki (kod szesnastkowy) */
    backgroundColor?: string
    /** Kolor obramowania odznaki (kod szesnastkowy) */
    borderColor?: string
    /** Kolor tekstu odznaki (kod szesnastkowy) */
    textColor?: string
    /** Dodatkowa klasa CSS do stylizacji */
    cssClass?: string
    /** Dla odznak weterana, próg czasu w milisekundach */
    veteranUserThresholdMillis?: number
    /** Czy ta odznaka jest wyświetlana w komentarzach użytkownika */
    displayedOnComments: boolean
    /** Kolejność wyświetlania odznaki */
    order?: number
}
[inline-code-end]
---