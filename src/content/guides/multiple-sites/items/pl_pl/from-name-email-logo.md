Czasami FastComments musi wysyłać e-maile do Twoich użytkowników, szczególnie jeśli nie używasz Secure SSO.

Przykłady obejmują weryfikację ich konta lub aktywności przy dodawaniu komentarza po raz pierwszy. FastComments będzie też wysyłać powiadomienia o odpowiedziach na ich komentarze.

Gdy FastComments wysyła e-maile do Twoich użytkowników, jako domyślnej nazwy nadawcy i adresu e-mail użyjemy `FastComments Robot` i `noreply@fastcomments.com`.

W stopce tych e-maili użyjemy także naszego logo.

Jeśli masz FastComments Flex lub Pro, wszystko to można dostosować dla każdej domeny oddzielnie za pomocą strony "My Domains":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Podczas dostosowywania logo wyświetlanego w e-mailach upewnij się, że rozmiar, który przesyłasz, jest taki sam, jaki chcesz pokazać w stopce e-maila.

### Podczas dostosowywania `From Domain`

Jeśli skonfigurujesz `From Domain`, dostawcy i klienci poczty muszą wiedzieć, że FastComments jest upoważniony do wysyłania wiadomości w Twoim imieniu. W przeciwnym razie określenie `From Domain` bez wykonania poniższych kroków najprawdopodobniej spowoduje, że wiadomości trafią do spamu.

#### 1. Konfiguracja SPF

Aby FastComments mogło bezpiecznie wysyłać wiadomości e-mail jako Twoja domena, dodaj rekord SPF, który nam to umożliwi.

Upewnij się, że istnieją rekordy SPF pozwalające `mail.fastcomments.com` i `sib.fastcomments.com` na wysyłanie poczty w imieniu Twojej domeny.

Więcej informacji, jak to zrobić, znajdziesz tutaj: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Konfiguracja DKIM

Oprócz SPF powinieneś skonfigurować DKIM. Gdy konfiguracja DNS będzie gotowa, możesz kliknąć "Show Advanced" na stronie konfiguracji domen, aby wyświetlić ustawienia DKIM dla każdej domeny.

Możesz też [wywołać API](/guide-api.html#domain-config-structure), aby ustawić konfigurację DKIM.

### Linki wypisania

Jeśli używasz SSO, funkcje wypisania używane w e-mailach i powiadomieniach można dostosować [przez DomainConfigs API](/guide-api.html#domain-config-structure).

### Maskowanie linków w e-mailach

Jeśli reputacja domeny Twojej strony powoduje, że powiadomienia e-mail trafiają do spamu, możesz przekierować przyciski "view comment" przez `fastcomments.com` zamiast linkować bezpośrednio do Twojej strony. Dostawcy skrzynek pocztowych oceniają każdy link w treści e-maila względem reputacji miejsca docelowego, więc gdy Twoja domena jest oznaczana, same linki zwiększają wynik spamu niezależnie od tego, jak poprawne jest Twoje ustawienie wysyłki.

Włącz to, klikając "Show Advanced" na stronie My Domains, w sekcji "Email Link Obfuscation". Ustawienie jest przypisane do każdej domeny osobno.

Po włączeniu linki w wiadomościach typu mention, reply, new-comment, subscribed-page, profile-comment i digest są przepisywane na krótkie tokeny, które przekierowują do oryginalnej strony po kliknięciu. Cel jest powiązany z Twoim tenantem: przekierowanie przekazuje dalej jedynie do URL-i, których host pasuje do jednej z Twoich skonfigurowanych domen, a tokeny wygasają automatycznie po 30 dniach.

Doświadczenie po kliknięciu pozostaje niezmienione. Czytelnicy nadal trafiają na Twoją stronę z komentarzem przewiniętym do widoku.