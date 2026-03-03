[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO używa szyfrowania HMAC-SHA256 jako mechanizmu implementacji SSO. Najpierw omówimy ogólną architekturę, podamy przykłady i szczegółowe kroki.

Istnieje także dokumentacja dotycząca migracji od innych dostawców z podobnymi mechanizmami SSO oraz różnic.

Przepływ wygląda następująco:

<div class="screenshot white-bg">
    <div class="title">Secure SSO Flow</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Secure SSO Diagram" />
</div>

Ponieważ Secure SSO wymaga pracy full-stack, kompletne działające przykłady kodu w Java/Spring, NodeJS/Express oraz czystym PHP są obecnie dostępne na <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">GitHubie</a>.

Chociaż w przykładzie NodeJS używamy ExpressJS, a w przykładzie Java — Spring, to w tych środowiskach nie są wymagane żadne dodatkowe frameworki/biblioteki do wdrożenia FastComments SSO — wystarczają natywne pakiety kryptograficzne.

Nie musisz tworzyć żadnych nowych endpointów API przy użyciu FastComments SSO. Po prostu zaszyfruj informacje o użytkowniku za pomocą swojego klucza tajnego i przekaż ładunek do widżetu komentarzy.

#### Pobierz swój tajny klucz API

Twój tajny klucz API można pobrać z <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">tej strony</a>. Możesz też znaleźć tę stronę, przechodząc do My Account, klikając kafelek API/SSO, a następnie klikając "Get API Secret Key".

#### Parametry widżetu komentarzy

Dokumentacja wysokiego poziomu API dla widżetu komentarzy jest dostępna <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">tutaj</a>.

Przejdźmy do bardziej szczegółowego omówienia, co oznaczają te parametry.

Widżet komentarzy przyjmuje obiekt konfiguracji — już przekazujesz go, jeśli używasz FastComments do przekazania identyfikatora klienta (nazywanego tenantId).

Aby włączyć SSO, przekaż nowy obiekt "sso", który musi zawierać następujące parametry. Wartości powinny być generowane po stronie serwera.

- userDataJSONBase64: Dane użytkownika w formacie JSON, które następnie są kodowane Base64.
- verificationHash: HMAC-SHA256 hash utworzony z UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Znacznik czasu epoki, w **milisekundach**. Nie może być z przyszłości ani starszy niż dwa dni.
- loginURL: URL, który widżet komentarzy może wyświetlić, aby zalogować użytkownika.
- logoutURL: URL, który widżet komentarzy może wyświetlić, aby wylogować użytkownika.
- loginCallback: Gdy jest dostarczone zamiast login URL, funkcja, którą widżet komentarzy wywoła po kliknięciu przycisku logowania.
- logoutCallback: Gdy jest dostarczone zamiast logout URL, funkcja, którą widżet komentarzy wywoła po kliknięciu przycisku wylogowania.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### Obiekt użytkownika

[inline-code-attrs-start title = 'Obiekt użytkownika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Wymagane. Maks. 1k znaków. **/
    id: string;
    /** Wymagane. Maks. 1k znaków. Uwaga: musi być unikalne. **/
    email: string;
    /** Wymagane. Maks. 1k znaków. Uwaga: nazwa użytkownika nie może być adresem e-mail. Nie musi być unikalna. **/
    username: string;
    /** Opcjonalne. Maks. 3k znaków dla URL-i. Domyślnie pobierane z Gravatar na podstawie e-maila. Obsługuje obrazy zakodowane w base64, w takim przypadku limit to 50k znaków. **/ 
    avatar?: string;
    /** Opcjonalne. Domyślnie false. **/
    optedInNotifications?: boolean;
    /** Opcjonalne. Domyślnie false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcjonalne. Maks. 100 znaków. Ta etykieta będzie wyświetlana obok ich nazwy. Domyślnie Administrator/Moderator, gdy ma zastosowanie. **/
    displayLabel?: string;
    /** Opcjonalne. Maks. 500 znaków. To będzie wyświetlane zamiast nazwy użytkownika. **/
    displayName?: string;
    /** Opcjonalne. Maks. 2k znaków. Nazwa użytkownika będzie linkować do tego adresu. **/
    websiteUrl?: string;
    /** Opcjonalne. Do 100 grup na użytkownika. Id grupy nie może być dłuższe niż 50 znaków. **/
    groupIds?: string[];
    /** Opcjonalne. Oznacza użytkownika jako administratora. **/
    isAdmin?: boolean;
    /** Opcjonalne. Oznacza użytkownika jako moderatora. **/
    isModerator?: boolean;
    /** Opcjonalne, domyślnie true. Ustaw na false, aby włączyć kartę „aktywność” w profilu użytkownika. **/
    isProfileActivityPrivate?: boolean;
    /** Opcjonalne, domyślnie false. Ustaw na true, aby wyłączyć komentarze w profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcjonalne, domyślnie false. Ustaw na true, aby wyłączyć wiadomości bezpośrednie do tego użytkownika. **/
    isProfileDMDisabled?: boolean;
    /** Opcjonalna konfiguracja odznak użytkownika. **/
    badgeConfig?: {
        /** Tablica globalnych identyfikatorów odznak do przypisania. Ograniczenie do 30 odznak. Kolejność jest zachowana. **/
        badgeIds: string[];
        /** Tablica identyfikatorów odznak przypisanych do bieżącej strony (urlId). Wyświetlane tylko na przypisanej stronie. **/
        pageBadgeIds?: string[];
        /** Jeśli true, zastępuje istniejące wyświetlane odznaki. Globalne i przypisane do strony są nadpisywane niezależnie. **/
        override?: boolean;
        /** Jeśli true, aktualizuje właściwości wyświetlania odznak z konfiguracji najemcy. **/
        update?: boolean;
    };
}
[inline-code-end]

#### Moderatorzy i administratorzy

Dla adminów i moderatorów przekaż odpowiednio flagi `isAdmin` lub `isModerator` w obiekcie `SSOUser`.

#### Powiadomienia

Aby włączyć lub wyłączyć powiadomienia, ustaw wartość `optedInNotifications` na `true` lub `false` odpowiednio. Przy pierwszym załadowaniu strony przez użytkownika z tą wartością w ładunku SSO jego ustawienia powiadomień zostaną zaktualizowane.

Dodatkowo, jeśli chcesz, aby użytkownicy otrzymywali e-maile z powiadomieniami o aktywności na stronach, na które się zapisali (zamiast tylko powiadomień w aplikacji), ustaw `optedInSubscriptionNotifications` na `true`.

#### Użytkownicy VIP i specjalne etykiety

Możesz wyświetlić specjalną etykietę obok nazwy użytkownika, używając opcjonalnego pola "displayLabel".

#### Niezalogowani użytkownicy

Aby przedstawić użytkownika niezalogowanego, po prostu nie wypełniaj userDataJSONBase64, verificationHash ani timestamp. Podaj loginURL.

Tacy użytkownicy nie będą mogli komentować; zamiast tego zostanie im pokazany komunikat logowania (wiadomość, link lub przycisk, w zależności od konfiguracji).

#### Bezpośrednie przykłady serializacji i hashowania danych użytkownika

Więcej szczegółów i przykładów <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">tutaj</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">tutaj</a> (java) i <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">tutaj</a> (php).

Rozumiemy, że każda integracja może być skomplikowanym i bolesnym procesem. Nie wahaj się skontaktować ze swoim przedstawicielem lub skorzystać ze <a href="https://fastcomments.com/auth/my-account/help" target="_blank">strony wsparcia</a>.

---