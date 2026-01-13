[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Podczas renderowania wątku komentarzy lub zostawiania komentarza, FastComments musi wiedzieć, do jakiej strony, artykułu lub produktu
należą te komentarze.

Aby to zrobić, używamy czegoś, co nazywamy "URL ID". Jest to albo identyfikator, taki jak ciąg znaków lub liczba, albo URL.

Domyślnie, jeśli nie określisz urlId, stanie się on adresem URL strony. Weźmiemy bieżący adres URL strony i oczyścimy go, aby usunąć
wszelkie typowe parametry marketingowe lub identyfikatory śledzące.

W przypadku integracji zewnętrznych, takich jak WordPress, nasza wtyczka zwykle użyje identyfikatora, który reprezentuje aktualnie wyświetlane informacje jako
URL ID, na przykład id artykułu/strony.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Jedną z rzeczy, do których często będziemy się odnosić w tym dokumencie, jest <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Interfejs dostosowywania widżetu</a>.

Ten interfejs można wykorzystać do wprowadzenia wielu zmian w widżecie komentarzy bez użycia kodu.

Tworząc regułę dostosowywania, często będziemy chcieli, aby dotyczyła wszystkich stron naszej witryny. Jednak w niektórych przypadkach chcemy spersonalizować widżet komentarzy
na konkretnej stronie — na przykład zastosować niestandardowe style lub sprawić, że komentarze dla tej strony będą anonimowe. Możesz również, na przykład, sprawić, że komentarze na żywo
pojawiają się od razu na niektórych stronach, podczas gdy na innych są ukryte pod przyciskami powiadomień.

Wszystko to jest możliwe dzięki polu wejściowemu URL ID na tej stronie, które wygląda następująco:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

Wartość w tym polu powinna odpowiadać parametrowi *urlId* przekazanemu do widżetu komentarzy. Jeśli chcesz, aby twoja reguła dostosowywania była niezależna od *urlId*, pozostaw to pole puste lub wpisz *.

As of 2023 the `URL ID` field in widget customization now also takes patterns! For example you may
have `*/blog/*` to add styling specific to your blog and `*/store/*` to have styling specific to your store,
all while using the same domain.

### Pułapki

1. Jeśli Twoja strona ma parametry hash (na przykład example.com#page-1) - domyślnie staną się one częścią URL ID.
2. Podczas migracji, na przykład z WordPress do Gatsby, może być konieczne zmigrowanie wartości komentarzy URL ID po początkowej migracji. W takim przypadku skontaktuj się z nami.