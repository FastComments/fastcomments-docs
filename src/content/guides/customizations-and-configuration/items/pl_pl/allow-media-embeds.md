Domyślnie FastComments nie zezwala na iframy w komentarzach. Po włączeniu osadzania mediów, komentujący mogą wkleić kod osadzenia (fragment `<iframe>`) od zaufanych dostawców, takich jak YouTube, Vimeo, SoundCloud i Spotify, i zostanie on wyrenderowany bezpośrednio w komentarzu.

Ze względów bezpieczeństwa nie jest to flaga konfiguracji widgetu po stronie klienta. To ustawienie po stronie serwera, walidowane przy zapisywaniu każdego komentarza, więc nie można go włączyć z poziomu strony. Dozwolone są tylko iframy wskazujące na wbudowaną listę zaufanych dostawców. Każdy inny iframe jest usuwany.

To odbywa się bez kodu, na stronie dostosowywania widgetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Dodawanie własnych dostawców

Jeśli chcesz zezwolić na osadzanie z dostawcy, który nie znajduje się na wbudowanej liście zaufanych, dodaj jego nazwę hosta w polu "Dodatkowe domeny osadzeń" na tej samej stronie. Te nazwy hostów są dozwolone dodatkowo do wbudowanych dostawców. Dopasowanie jest dokładne, więc podaj pełną nazwę hosta (na przykład player.example.com). Wszystko, czego nie wymienisz, pozostanie zablokowane.

Zarówno zwykłe pole komentarza, jak i edytor WYSIWYG obsługują wklejanie osadzenia. W edytorze WYSIWYG osadzenie jest wstawiane jako blok, który można usunąć.