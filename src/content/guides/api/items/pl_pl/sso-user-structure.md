FastComments provides an easy to use SSO solution. Updating a user's information with the HMAC-based integration is
as simple as having the user load the page with an updated payload.

However, it may be desirable to manage a user outside that flow, to improve consistency of your application.

The SSO User API provides a way to CRUD objects that we call SSOUsers. These objects are different from regular Users and
kept separate for type safety.

The structure for the SSOUser object is as follows:

[inline-code-attrs-start title = 'Struktura SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // Uprawnienie administratora - użytkownicy SSO z tą flagą są rozliczani jako administratorzy SSO (oddzielnie od zwykłych użytkowników SSO)
    isAdminAdmin?: boolean // Uprawnienie administratora - użytkownicy SSO z tą flagą są rozliczani jako administratorzy SSO (oddzielnie od zwykłych użytkowników SSO)
    isCommentModeratorAdmin?: boolean // Uprawnienie moderatora - użytkownicy SSO z tą flagą są rozliczani jako moderatorzy SSO (oddzielnie od zwykłych użytkowników SSO)
    /** Jeśli null, Kontrola Dostępu nie zostanie zastosowana do użytkownika. Jeśli pusta lista, ten użytkownik nie będzie mógł widzieć żadnych stron ani używać @mention wobec innych użytkowników. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Nie pozwalaj innym użytkownikom widzieć aktywności tego użytkownika, w tym komentarzy, na jego profilu. Domyślnie true, aby zapewnić bezpieczne profile. **/
    isProfileActivityPrivate?: boolean
    /** Nie pozwalaj innym użytkownikom zostawiać komentarzy na profilu tego użytkownika ani widzieć istniejących komentarzy na profilu. Domyślnie false. **/
    isProfileCommentsPrivate?: boolean
    /** Nie pozwalaj innym użytkownikom wysyłać prywatnych wiadomości do tego użytkownika. Domyślnie false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opcjonalna konfiguracja odznak użytkownika. **/
    badgeConfig?: {
        /** Tablica identyfikatorów odznak przypisanych użytkownikowi. Ograniczenie do 30 odznak. Kolejność zachowana. **/
        badgeIds: string[]
        /** Jeśli true, zastępuje wszystkie istniejące wyświetlane odznaki dostarczonymi. Jeśli false lub pominięte, dodaje do istniejących odznak. **/
        override?: boolean
        /** Jeśli true, aktualizuje właściwości wyświetlania odznak z konfiguracji najemcy. **/
        update?: boolean
    }
}
[inline-code-end]

### Billing for SSO Users

SSO users are billed differently based on their permission flags:

- **Zwykli użytkownicy SSO**: Users without admin or moderator permissions are billed as regular SSO users
- **Administratorzy SSO**: Users with `isAccountOwner` or `isAdminAdmin` flags are billed separately as SSO Admins (same rate as regular tenant admins)
- **Moderatorzy SSO**: Users with `isCommentModeratorAdmin` flag are billed separately as SSO Moderators (same rate as regular moderators)

**Ważne**: Aby zapobiec podwójnemu naliczaniu, system automatycznie deduplikuje użytkowników SSO względem zwykłych użytkowników i moderatorów konta na podstawie adresu e-mail. Jeśli użytkownik SSO ma ten sam adres e-mail co zwykły użytkownik konta lub moderator, nie zostanie naliczony dwukrotnie.

### Access Control

Użytkownicy mogą być podzieleni na grupy. Do tego służy pole `groupIds` i jest ono opcjonalne.

### @Wzmianki

Domyślnie `@mentions` używają `username` do wyszukiwania innych użytkowników SSO, gdy wpisany zostanie znak `@`. Jeśli użyty jest `displayName`, wyniki pasujące do `username` zostaną zignorowane, gdy istnieje zgodność z `displayName`, a wyniki wyszukiwania `@mention` będą używać `displayName`.

### Subscriptions

W FastComments użytkownicy mogą subskrybować stronę, klikając ikonę dzwonka w widgetcie komentarzy i wybierając Subskrybuj.

W przypadku zwykłego użytkownika wysyłamy mu e-maile z powiadomieniami na podstawie jego ustawień powiadomień.

W przypadku użytkowników SSO rozdzielamy to dla zachowania kompatybilności wstecz. Użytkownicy otrzymają dodatkowe e-maile z powiadomieniami o subskrypcji tylko jeśli ustawisz `optedInSubscriptionNotifications` na `true`.

### Badges

Możesz przypisać odznaki użytkownikom SSO za pomocą właściwości `badgeConfig`. Odznaki to wizualne wskaźniki pojawiające się obok nazwy użytkownika w komentarzach.

- `badgeIds` - Tablica identyfikatorów odznak, które zostaną przypisane użytkownikowi. Muszą to być prawidłowe identyfikatory odznak utworzone na Twoim koncie FastComments. Ograniczenie do 30 odznak.
- `override` - Jeśli `true`, wszystkie istniejące odznaki wyświetlane w komentarzach zostaną zastąpione dostarczonymi. Jeśli `false` lub pominięte, dostarczone odznaki zostaną dodane do istniejących.
- `update` - Jeśli `true`, właściwości wyświetlania odznak będą aktualizowane z konfiguracji konta za każdym razem, gdy użytkownik się zaloguje.

---