Czasami FastComments musi wysyłać e‑maile do Twoich użytkowników, szczególnie jeśli nie używasz Secure SSO.

Przykłady obejmują weryfikację ich konta lub aktywności przy pierwszym komentowaniu. FastComments będzie również wysyłać im powiadomienia o odpowiedziach na ich komentarze.

Gdy FastComments wysyła e‑maile do Twoich użytkowników, użyjemy domyślnej nazwy nadawcy i adresu e‑mail: `FastComments Robot` oraz `noreply@fastcomments.com`.

Użyjemy również naszego własnego logo w stopce tych e‑maili.

Jeśli masz FastComments Flex lub Pro, wszystko to można dostosować na poziomie domeny za pomocą strony „My Domains”:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='Formularz ustawień e‑maili per domena z polami Nazwa nadawcy, E‑mail nadawcy i przesyłania logo'; title='Dostosowywanie nazwy nadawcy, e‑maila i logo' app-screenshot-end]

Podczas dostosowywania logo wyświetlanego w e‑mailach, upewnij się, że rozmiar, który przesyłasz, jest taki sam, jak rozmiar, który chcesz wyświetlić w stopce e‑maila.

### Podczas dostosowywania `From Domain`

Jeśli dostosujesz `From Domain`, dostawcy poczty e‑mail i klienci muszą wiedzieć, że FastComments jest upoważniony do wysyłania e‑maili w Twoim imieniu. W przeciwnym razie, określenie `From Domain` bez wykonania poniższych kroków prawdopodobnie spowoduje, że e‑maile trafią do spamu.

#### 1. Konfiguracja SPF

Aby umożliwić FastComments bezpieczne wysyłanie e‑maili jako Twoja domena, upewnij się, że dodasz rekord SPF, który nam to pozwala.

Upewnij się, że istnieją rekordy SPF pozwalające `mail.fastcomments.com` i `sib.fastcomments.com` wysyłać pocztę jako Twoja domena.

Więcej informacji, jak to zrobić, znajdziesz tutaj: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Konfiguracja DKIM

Oprócz SPF, powinieneś skonfigurować DKIM. Gdy konfiguracja DNS będzie gotowa, możesz kliknąć „Show Advanced” na stronie konfiguracji domen, aby wyświetlić ustawienia DKIM per domena.

Możesz również [wywołać API](/guide-api.html#domain-config-structure), aby ustawić konfigurację DKIM.

### Linki do wypisania się

Podczas korzystania z SSO, funkcje wypisania się używane w e‑mailach i powiadomieniach można dostosować [za pośrednictwem DomainConfigs API](/guide-api.html#domain-config-structure).

### Maskowanie linków w e‑mailach

Jeśli reputacja domeny Twojej witryny powoduje, że e‑maile z powiadomieniami trafiają do spamu, możesz kierować przyciski „view comment” przez `fastcomments.com` zamiast linkować bezpośrednio do swojej strony. Dostawcy skrzynek pocztowych oceniają każdy link w treści e‑maila pod kątem reputacji docelowego adresu, więc gdy Twoja domena jest oznaczona, same linki przyczyniają się do wyniku spamu, niezależnie od tego, jak czysta jest Twoja konfiguracja wysyłki.

Włącz to w sekcji „Show Advanced” na stronie My Domains, w sekcji „Email Link Obfuscation”. Ustawienie jest per domena.

Po włączeniu, linki w e‑mailach typu mention, reply, new-comment, subscribed-page, profile-comment i digest są przekształcane w krótkie tokeny, które po kliknięciu przekierowują do oryginalnej strony. Docelowy adres jest powiązany z Twoim najemcą: przekierowanie działa tylko do URL‑ów, których host pasuje do jednej z Twoich skonfigurowanych domen, a tokeny automatycznie wygasają po 30 dniach.

Doświadczenie po kliknięciu pozostaje niezmienione. Czytelnicy nadal trafiają na Twoją stronę, a komentarz jest przewinięty do widoku.

---