FastComments uwierzytelnia żądania do Twojego konta, aby sprawdzić, że pochodzą one z Twojej witryny. Dlatego  
musimy wiedzieć, którą witrynę lub witryny chcesz zainstalować FastComments.

FastComments obsługuje uwierzytelnianie za pomocą domeny, a także subdomen.

Weźmy pod uwagę witrynę `https://example.com`. W tym przypadku "`example.com`" jest domeną. `example.com` obsługuje zarówno `example.com`, jak i `www.example.com`. Nazwiemy "www" "subdomeną".

Na przykład:

- Aby zezwolić tylko na `blog.example.com`:
  - Dodaj `blog.example.com` do swoich domen.
- Aby zezwolić na `www.example.com`, `somesite.example.com` i `example.com`:
  - Dodaj `example.com` do swoich domen.
  - To jest rozliczane jako **jedna domena** powiązana z Twoim kontem.
- Możesz teraz dodać subdomeny wieloznaczne, na przykład *myname.vercel.app.  
  - To jest rozliczane jako **jedna domena** powiązana z Twoim kontem.

Jeśli korzystałeś z platformy blogowej i otrzymałeś subdomenę, powinieneś dodać **pełną domenę wraz z subdomeną** do swojego konta, na przykład: `cats.blogger.com`.

Możemy dodać domeny do naszego konta, odwiedzając stronę `My Domains` i klikając `Add a Domain` na dole:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content'; alt='Strona Moje domeny wyświetlająca domeny na koncie, z przyciskiem Dodaj domenę na dole'; title='Strona Moje domeny' app-screenshot-end]

W trakcie okresu próbnego, **domeny są automatycznie dodawane do Twojego konta**, gdy żądania pochodzą z tych domen. Jednak po tym czasie muszą być dodane ręcznie ze względów bezpieczeństwa. Powinieneś otrzymać e‑mail, gdy to automatyczne zachowanie wystąpi.

Nie musisz **dodawać** `localhost` do lokalnego rozwoju – jest on domyślnie dozwolony.

#### Przez API

Domeny mogą być również dodawane i konfigurowane [przez API DomainConfigs](/guide-api.html#domain-config-structure).