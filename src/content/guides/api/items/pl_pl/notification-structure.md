Obiekt `Notification` reprezentuje powiadomienie dla użytkownika.

Obiekty `Notification` są tworzone automatycznie i nie można ich tworzyć przez API. Wygasają też po roku.
Powiadomień nie można usuwać. Można jednak zaktualizować je, ustawiając `viewed` na `false`, i można filtrować zapytania po `viewed`.

Użytkownik może również zrezygnować z powiadomień dla konkretnego komentarza, ustawiając `optedOut` w powiadomieniu na `true`. Można ponownie wyrazić zgodę, ustawiając je na `false`.

Istnieją różne typy powiadomień - sprawdź `relatedObjectType` i `type`.

Sposoby tworzenia powiadomień są dość elastyczne i mogą być wyzwalane w wielu scenariuszach (zob. `NotificationType`).

Na dzień dzisiejszy istnienie `Notification` niekoniecznie oznacza, że e-mail został lub powinien zostać wysłany. Raczej powiadomienia
są używane do kanału powiadomień i powiązanych integracji.

Struktura obiektu `Notification` jest następująca:

[inline-code-attrs-start title = 'Struktura powiadomienia'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum NotificationObjectType {
    Comment = 0,
    Profile = 1,
    Tenant = 2
}

enum NotificationType {
    /** Jeśli ktoś odpowiedział na Twój komentarz. **/
    RepliedToMe = 0,
    /** Jeśli ktoś odpowiedział gdziekolwiek w wątku (nawet w odpowiedziach zagnieżdżonych) w wątku, w którym skomentowałeś. **/
    RepliedTransientChild = 1,
    /** Jeśli Twój komentarz otrzymał głos poparcia. **/
    VotedMyComment = 2,
    /** Jeśli na głównym wątku strony, na którą jesteś subskrybowany, zostanie dodany nowy komentarz. **/
    SubscriptionReplyRoot = 3,
    /** Jeśli ktoś skomentował Twój profil. **/
    CommentedOnProfile = 4,
    /** Jeśli masz wiadomość prywatną. **/
    DirectMessage = 5,
    /** TrialLimits dotyczy tylko użytkowników tenant. **/
    TrialLimits = 6,
    /** Jeśli zostałeś wspomniany przy użyciu @. **/
    Mentioned = 7
}

interface Notification {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId?: string
    /** Przy pracy z SSO wystarczy zwrócić uwagę na `userId`. **/
    anonUserId?: string
    /** urlId jest prawie zawsze zdefiniowany. Jest opcjonalny tylko dla powiadomień na poziomie tenant, które są rzadkie. **/
    urlId?: string
    /** URL jest buforowany dla szybkiej nawigacji do źródła powiadomienia. **/
    url?: string
    /** Tytuł strony jest buforowany dla szybkiego odczytania źródła powiadomienia. **/
    pageTitle?: string
    relatedObjectType: NotificationObjectType
    /** Na przykład id komentarza. **/
    relatedObjectId: string
    viewed: boolean
    createdAt: string // ciąg reprezentujący datę
    type: NotificationType
    fromCommentId?: string
    fromVoteId?: string
    /** fromUserName i fromUserAvatarSrc są buforowane tutaj dla szybkiego wyświetlania powiadomienia. Są aktualizowane, gdy użytkownik zostanie zaktualizowany. **/
    fromUserName: string
    fromUserId: string
    fromUserAvatarSrc?: string
    /** Ustaw to na true, aby przestać otrzymywać powiadomienia dla tego obiektu. **/
    optedOut?: boolean
}
[inline-code-end]