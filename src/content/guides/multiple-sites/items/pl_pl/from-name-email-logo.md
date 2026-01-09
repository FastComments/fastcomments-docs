---
Czasami FastComments musi wysyłać e-maile do Twoich użytkowników, zwłaszcza jeśli nie korzystasz z Secure SSO.

Przykłady obejmują weryfikację ich konta lub aktywności przy komentowaniu po raz pierwszy. FastComments wyśle im również powiadomienia o odpowiedziach na ich komentarze.

Kiedy FastComments wysyła e-maile do Twoich użytkowników, użyjemy domyślnej nazwy nadawcy i adresu e-mail `FastComments Robot` oraz `noreply@fastcomments.com`.

W stopce tych e-maili użyjemy także naszego logo.

Jeśli masz FastComments Flex lub Pro, wszystko to można dostosować dla każdej domeny za pomocą strony "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

Podczas dostosowywania logo wyświetlanego w e-mailach upewnij się, że rozmiar, który przesyłasz, jest taki sam, jakiego chcesz użyć w stopce e-maila.

### Przy dostosowywaniu `From Domain`

Jeśli spersonalizujesz `From Domain`, dostawcy i klienci poczty muszą wiedzieć, że FastComments jest uprawniony do wysyłania e-maili w Twoim imieniu. W przeciwnym razie, zdefiniowanie `From Domain` bez wykonania poniższych kroków prawdopodobnie spowoduje, że e-maile trafią do spamu.

#### 1. Konfiguracja SPF

Aby umożliwić FastComments bezpieczne wysyłanie e-maili w imieniu Twojej domeny, dodaj rekord SPF, który nam na to pozwoli.

Upewnij się, że istnieją rekordy SPF pozwalające `mail.fastcomments.com` i `sib.fastcomments.com` na wysyłanie poczty w imieniu Twojej domeny.

Some more information on how to do this is here: https://mailtrap.io/blog/multiple-spf-records/

#### 2. Konfiguracja DKIM

Oprócz SPF powinieneś skonfigurować DKIM. Gdy konfiguracja DNS będzie gotowa, możesz kliknąć "Show Advanced" na stronie konfiguracji domen, aby wyświetlić ustawienia DKIM dla każdej domeny.

Możesz także [wywołać API](/guide-api.html#domain-config-structure), aby ustawić konfigurację DKIM.

### Linki do wypisania

Jeśli korzystasz z SSO, funkcje wypisywania używane w e-mailach i powiadomieniach można dostosować [za pomocą DomainConfigs API](/guide-api.html#domain-config-structure).

---