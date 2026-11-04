Domyślnie FastComments nie zezwala na ramki iframe w komentarzach. Po włączeniu osadzania mediów komentujący mogą wkleić kod osadzania (fragment `<iframe>`) z zaufanych dostawców, takich jak YouTube, Vimeo, SoundCloud i Spotify, i zostanie on wyrenderowany w komentarzu.

Ze względów bezpieczeństwa nie jest to flaga konfiguracji widżetu po stronie klienta. To ustawienie po stronie serwera, walidowane przy zapisie każdego komentarza, więc nie można go włączyć ze strony. Dozwolone są tylko ramki iframe wskazujące na wbudowaną listę zaufanych dostawców. Każda inna ramka iframe jest usuwana.

Robi się to bez kodu, na stronie dostosowywania widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Ustawienie osadzania mediów włączone na stronie dostosowywania widżetu, pozwalające komentującym wklejać zaufane ramki iframe'; title='Zezwalaj na osadzanie mediów' app-screenshot-end]

### Dodawanie własnych dostawców

Jeśli chcesz zezwolić na osadzanie od dostawcy, którego nie ma na wbudowanej liście zaufanych, dodaj jego nazwę hosta w polu "Additional Embed Domains" na tej samej stronie. Te nazwy hostów są dozwolone dodatkowo do wbudowanych dostawców. Dopasowanie jest dokładne, więc podaj pełną nazwę hosta (na przykład player.example.com). Wszystko, czego nie wymienisz, pozostaje zablokowane.

Zarówno zwykłe pole komentarza, jak i edytor WYSIWYG obsługują wklejanie osadzenia. W edytorze WYSIWYG osadzenie jest wstawiane jako blok, który można usunąć.