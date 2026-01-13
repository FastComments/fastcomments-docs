[related-parameter-start name = 'sso'; type = 'FastCommentsSSO'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1' related-parameter-end]

FastComments Secure SSO używa szyfrowania HMAC-SHA256 jako mechanizmu implementacji SSO. Najpierw omówimy ogólną architekturę, przedstawimy przykłady i szczegółowe kroki.

Istnieje również dokumentacja dotycząca migracji z innych dostawców z podobnymi mechanizmami SSO oraz różnic.

Przepływ wygląda tak:

<div class="screenshot white-bg">
    <div class="title">Bezpieczny przepływ SSO</div>
    <img class="screenshot-image" src="/images/secure-sso-diagram.svg" alt="Schemat Secure SSO" />
</div>

Ponieważ Secure SSO obejmuje rozwój full-stack, pełne działające przykłady kodu w Java/Spring, NodeJS/Express i czystym PHP są obecnie <a href="https://github.com/FastComments/fastcomments-code-examples/tree/master/sso" target="_blank">na GitHub</a>.

Chociaż w przykładzie NodeJS używamy ExpressJS, a w przykładzie Java używamy Spring, w tych środowiskach uruchomieniowych nie są wymagane żadne frameworki/biblioteki do implementacji FastComments SSO — działają natywne pakiety kryptograficzne.

Nie musisz tworzyć żadnych nowych punktów końcowych API przy użyciu FastComments SSO. Po prostu zaszyfruj informacje użytkownika za pomocą swojego klucza sekretnego i przekaż ładunek do widgetu komentarzy.

#### Pobierz swój tajny klucz API

Twój tajny klucz API można pobrać z <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">tej strony</a>. Możesz także znaleźć tę stronę, przechodząc do Moje konto, klikając kafelek API/SSO, a następnie klikając "Get API Secret Key".

#### Parametry widgetu komentarzy

Wysokopoziomowa dokumentacja API dla widgetu komentarzy jest dostępna <a href="https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L1" target="_blank">tutaj</a>.

Przejdźmy do bardziej szczegółowego opisu, co oznaczają te parametry.

Widget komentarzy przyjmuje obiekt konfiguracyjny — przekazujesz go już, jeśli używasz FastComments do przekazania identyfikatora klienta (nazywanego tenantId).

Aby włączyć SSO, przekaż nowy obiekt "sso", który musi zawierać następujące parametry. Wartości powinny być generowane po stronie serwera.

- userDataJSONBase64: Dane użytkownika w formacie JSON, które następnie są kodowane w Base64.
- verificationHash: HMAC-SHA256 hash utworzony z UNIX_TIME_MILLIS + userDataJSONBase64.
- timestamp: Znacznik czasu epoch w **milisekundach**. Nie może być w przyszłości ani starszy niż dwa dni wstecz.
- loginURL: URL, który widget komentarzy może wyświetlić, aby zalogować użytkownika.
- logoutURL: URL, który widget komentarzy może wyświetlić, aby wylogować użytkownika.
- loginCallback: Gdy jest podany zamiast loginURL, funkcja, którą widget komentarzy wywoła po kliknięciu przycisku logowania.
- logoutCallback: Gdy jest podany zamiast logoutURL, funkcja, którą widget komentarzy wywoła po kliknięciu przycisku wylogowania.

[code-example-start config = {sso: { userDataJSONBase64: '...', verificationHash: '...', timestamp: Date.now(), loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', loginCallback: function() { console.log('Log the user in here...'); }, logoutCallback: function() { console.log('Log the user out here...') } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12]; title = 'Secure SSO Client Code'; isFunctional = false; code-example-end]

#### The User Object

The User object contains the following schema:
[inline-code-attrs-start title = 'Obiekt użytkownika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    /** Wymagane. Maks. 1k znaków. **/
    id: string;
    /** Wymagane. Maks. 1k znaków. Uwaga: musi być unikalny. **/
    email: string;
    /** Wymagane. Maks. 1k znaków. Uwaga: nazwa użytkownika nie może być adresem e-mail. Nie musi być unikalna. **/
    username: string;
    /** Opcjonalne. Maks. 3k znaków dla URL-i. Domyślnie pobierane z gravatar na podstawie e-maila. Obsługuje obrazy zakodowane w base64, w takim przypadku limit to 50k znaków. **/ 
    avatar?: string;
    /** Opcjonalne. Domyślnie false. **/
    optedInNotifications?: boolean;
    /** Opcjonalne. Domyślnie false. **/
    optedInSubscriptionNotifications?: boolean;
    /** Opcjonalne. Maks. 100 znaków. Ta etykieta będzie wyświetlana obok ich nazwy. Domyślnie Administrator/Moderator, gdy ma zastosowanie. **/
    displayLabel?: string;
    /** Opcjonalne. Maks. 500 znaków. To będzie wyświetlane zamiast nazwy użytkownika. **/
    displayName?: string;
    /** Opcjonalne. Maks. 2k znaków. Nazwa użytkownika będzie prowadzić do tego adresu. **/
    websiteUrl?: string;
    /** Opcjonalne. Do 100 grup na użytkownika. ID grupy nie może mieć więcej niż 50 znaków. **/
    groupIds?: string[];
    /** Opcjonalne. Oznacza użytkownika jako administratora. **/
    isAdmin?: boolean;
    /** Opcjonalne. Oznacza użytkownika jako moderatora. **/
    isModerator?: boolean;
    /** Opcjonalne, domyślnie true. Ustaw na false, aby włączyć kartę "activity" w profilu użytkownika. **/
    isProfileActivityPrivate?: boolean;
    /** Opcjonalne, domyślnie false. Ustaw na true, aby wyłączyć komentarze w profilu. **/
    isProfileCommentsPrivate?: boolean;
    /** Opcjonalne, domyślnie false. Ustaw na true, aby wyłączyć wysyłanie prywatnych wiadomości do tego użytkownika. **/
    isProfileDMDisabled?: boolean;
}
[inline-code-end]

#### Moderatorzy i administratorzy

Dla administratorów i moderatorów przekaż odpowiednio flagi `isAdmin` lub `isModerator` w obiekcie `SSOUser`.

#### Powiadomienia

Aby włączyć lub wyłączyć powiadomienia, ustaw wartość `optedInNotifications` na `true` lub `false`. Przy pierwszym załadowaniu strony przez użytkownika z tą wartością w ładunku SSO, jego ustawienia powiadomień zostaną zaktualizowane.

Dodatkowo, jeśli chcesz, aby użytkownicy otrzymywali e-maile z powiadomieniami o aktywności na stronach, na które się zapisali (w przeciwieństwie do samych powiadomień w aplikacji), ustaw `optedInSubscriptionNotifications` na `true`.

#### Użytkownicy VIP i specjalne etykiety

Możesz wyświetlić specjalną etykietę obok imienia użytkownika, używając opcjonalnego pola "displayLabel".

#### Niezalogowani użytkownicy

Aby przedstawić niezalogowanego użytkownika, po prostu nie wypełniaj userDataJSONBase64, verificationHash ani timestamp. Podaj loginURL.

Tacy użytkownicy nie będą mogli komentować; zamiast tego zostanie im wyświetlony komunikat logowania (wiadomość, link lub przycisk, w zależności od konfiguracji).

#### Bezpośrednie przykłady serializacji i haszowania danych użytkownika

Więcej szczegółów i przykłady <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/nodejs/routes/index.js#L26" target="_blank">tutaj</a> (js), <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/java/src/main/java/com/winricklabs/ssodemo/DemoController.java#L54" target="_blank">tutaj</a> (java) oraz <a href="https://github.com/fastcomments/fastcomments-code-examples/blob/master/sso/php/server.php#L27" target="_blank">tutaj</a> (php).

Rozumiemy, że każda integracja może być skomplikowana i uciążliwa. Nie wahaj się skontaktować ze swoim przedstawicielem lub skorzystać ze <a href="https://fastcomments.com/auth/my-account/help" target="_blank">strony wsparcia</a>.

---