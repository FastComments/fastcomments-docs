FastComments oferuje łatwe w użyciu rozwiązanie SSO. Aktualizacja informacji o użytkowniku w integracji opartej na HMAC jest tak prosta, jak załadowanie przez użytkownika strony ze zaktualizowanym payloadem.

Jednak może być pożądane zarządzanie użytkownikiem poza tym przepływem, aby poprawić spójność Twojej aplikacji.

SSO User API zapewnia sposób wykonywania operacji CRUD na obiektach, które nazywamy SSOUsers. Te obiekty różnią się od zwykłych Users i są przechowywane osobno dla bezpieczeństwa typów.

Struktura obiektu SSOUser wygląda następująco:

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
    isAccountOwner?: boolean // Uprawnienie administratora - użytkownicy SSO z tą flagą są rozliczani jako SSO Admins (oddzielnie od zwykłych użytkowników SSO)
    isAdminAdmin?: boolean // Uprawnienie administratora - użytkownicy SSO z tą flagą są rozliczani jako SSO Admins (oddzielnie od zwykłych użytkowników SSO)
    isCommentModeratorAdmin?: boolean // Uprawnienie moderatora - użytkownicy SSO z tą flagą są rozliczani jako SSO Moderators (oddzielnie od zwykłych użytkowników SSO)
    /** Jeśli null, Kontrola Dostępu nie będzie stosowana do użytkownika. Jeśli pusta lista, ten użytkownik nie będzie mógł widzieć żadnych stron ani @mentionować innych użytkowników. **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** Nie pozwól innym użytkownikom widzieć aktywności tego użytkownika, w tym komentarzy, na jego profilu. Domyślnie true, aby zapewnić bezpieczne profile domyślnie. **/
    isProfileActivityPrivate?: boolean
    /** Nie pozwól innym użytkownikom zostawiać komentarzy na profilu użytkownika ani widzieć istniejących komentarzy profilu. Domyślnie false. **/
    isProfileCommentsPrivate?: boolean
    /** Nie pozwól innym użytkownikom wysyłać wiadomości bezpośrednich do tego użytkownika. Domyślnie false. **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** Opcjonalna konfiguracja odznak użytkownika. **/
    badgeConfig?: {
        /** Tablica identyfikatorów odznak do przypisania użytkownikowi. Ograniczone do 30 odznak. Kolejność jest respektowana. To są globalne odznaki widoczne na wszystkich stronach. **/
        badgeIds: string[]
        /** Tablica identyfikatorów odznak ograniczonych do bieżącej strony (urlId). Te odznaki są wyświetlane tylko na stronie, na której zostały przypisane. **/
        pageBadgeIds?: string[]
        /** Jeśli true, zastępuje wszystkie istniejące wyświetlane odznaki podanymi. Globalne odznaki i odznaki ograniczone do strony są nadpisywane niezależnie. Jeśli false, dodaje do istniejących odznak. **/
        override?: boolean
        /** Jeśli true, aktualizuje właściwości wyświetlania odznak z konfiguracji tenanta. **/
        update?: boolean
    }
}
[inline-code-end]

### Rozliczanie użytkowników SSO

Użytkownicy SSO są rozliczani inaczej w zależności od ich flag uprawnień:

- **Zwykli użytkownicy SSO**: Użytkownicy bez uprawnień administratora lub moderatora są rozliczani jako zwykli użytkownicy SSO
- **SSO Admins**: Użytkownicy z flagami `isAccountOwner` lub `isAdminAdmin` są rozliczani oddzielnie jako SSO Admins (ta sama stawka co zwykli administratorzy tenanta)
- **SSO Moderators**: Użytkownicy z flagą `isCommentModeratorAdmin` są rozliczani oddzielnie jako SSO Moderators (ta sama stawka co zwykli moderatorzy)

**Ważne**: Aby zapobiec podwójnemu rozliczaniu, system automatycznie deduplikuje użytkowników SSO względem zwykłych użytkowników i moderatorów tenanta na podstawie adresu e-mail. Jeśli użytkownik SSO ma ten sam e-mail co zwykły użytkownik lub moderator tenanta, nie zostanie rozliczony dwukrotnie.

### Kontrola Dostępu

Użytkownicy mogą być dzieleni na grupy. Do tego służy pole `groupIds` i jest ono opcjonalne.

### @Wzmianki

Domyślnie `@mentions` będą używać `username` do wyszukiwania innych użytkowników sso po wpisaniu znaku `@`. Jeśli używane jest `displayName`, to wyniki pasujące do `username` będą ignorowane, gdy istnieje dopasowanie dla `displayName`, a wyniki wyszukiwania `@mention` będą używać `displayName`.

### Subskrypcje

W FastComments użytkownicy mogą subskrybować stronę, klikając ikonę dzwonka w widżecie komentarzy, a następnie klikając Subscribe.

W przypadku zwykłego użytkownika wysyłamy mu e-maile powiadomień na podstawie jego ustawień powiadomień.

W przypadku użytkowników SSO dzielimy to ze względu na kompatybilność wsteczną. Użytkownicy otrzymają te dodatkowe e-maile powiadomień o subskrypcji tylko wtedy, gdy ustawisz `optedInSubscriptionNotifications` na `true`.

### Odznaki

Możesz przypisywać odznaki użytkownikom SSO za pomocą właściwości `badgeConfig`. Odznaki to wizualne wskaźniki pojawiające się obok nazwy użytkownika w komentarzach.

- `badgeIds` - Tablica identyfikatorów odznak do przypisania użytkownikowi. To są globalne odznaki widoczne na wszystkich stronach. Muszą to być prawidłowe identyfikatory odznak utworzone w Twoim koncie FastComments. Ograniczone do 30 odznak.
- `pageBadgeIds` - Opcjonalna tablica identyfikatorów odznak ograniczonych do bieżącej strony (`urlId`). Te odznaki są wyświetlane tylko na stronie, na której zostały przypisane. Różne strony mogą mieć różne odznaki ograniczone do strony dla tego samego użytkownika.
- `override` - Jeśli true, wszystkie istniejące wyświetlane odznaki zostaną zastąpione podanymi. Globalne odznaki i odznaki ograniczone do strony są nadpisywane niezależnie — nadpisanie globalnych odznak nie wpływa na odznaki ograniczone do strony i odwrotnie. Jeśli false lub pominięte, podane odznaki zostaną dodane do istniejących odznak.
- `update` - Jeśli true, właściwości wyświetlania odznak będą aktualizowane z konfiguracji tenanta przy każdym logowaniu użytkownika.