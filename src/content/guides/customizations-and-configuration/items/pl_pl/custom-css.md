[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments został zaprojektowany z myślą o możliwościach dostosowywania. Sam widget komentarzy działa wewnątrz iframe ze względów bezpieczeństwa, więc aby zastosować
niestandardowe style, musisz postępować według jednego z dwóch podejść.

Pierwsze, najłatwiejsze podejście i przez nas preferowane, to skorzystanie ze [strony dostosowywania widgetu](https://fastcomments.com/auth/my-account/customize-widget).

W stronie dostosowywania widgetu zobacz sekcję "Show Advanced Options", pod którą znajduje się obszar oznaczony jako "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

To podejście ma pewne zalety:
1. Wprowadzony CSS jest minifikowany przed wysłaniem do użytkownika, a formatowanie pozostaje spójne w interfejsie edycji.
2. Otrzymujesz wszystkie korzyści z interfejsu personalizacji widgetu, na przykład łatwe dostosowywanie widgetu komentarzy w różny sposób dla różnych witryn.
3. Gdy wprowadzimy zmiany w widgetcie komentarzy, Twoje niestandardowe style zostaną przetestowane w ramach naszego procesu wydawniczego.

Drugie podejście polega na określeniu parametru **customCSS** w konfiguracji widgetu, w następujący sposób:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Jednak ma to *ograniczenia*:
1. Istnieje limit ilości niestandardowego CSS, który można przesłać, zanim nasze serwery odrzucą żądanie, ze względu na rozmiar nagłówków.
2. Musisz zarządzać niestandardowym CSS w swojej infrastrukturze i systemie budowania. Może to być zaletą, a nie wadą.
3. W tym przypadku istnieje dodatkowy narzut polegający na wysyłaniu niestandardowego CSS przez sieć **dwukrotnie**, ponieważ musi on zostać wysłany do naszych serwerów, a następnie zwrócony w zawartości iframe. Jednak dla większości rozmiarów danych przesyłanych nie jest to zauważalne.
4. Częstą optymalizacją jest minifikacja CSS, aby zmniejszyć jego rozmiar przesyłany przez sieć — przy tym podejściu musisz się tym jednak zająć samodzielnie.
5. Twój niestandardowy CSS nie będzie przez nas testowany przy wprowadzaniu zmian.

### Zewnętrzne pliki CSS

Możesz polecić widgetowi pobranie zewnętrznego pliku używając `@import`!

Zaleca się umieszczenie `@import` w regule dostosowującej. W ten sposób, jeśli kiedykolwiek będziemy musieli wprowadzić zmianę w widgetcie komentarzy, możemy użyć naszych narzędzi automatyzacji do weryfikacji Twojej konfiguracji. Na przykład utworzyłbyś regułę dostosowania w interfejsie personalizacji widgetu, kliknął `Advanced` i wpisał w `Custom CSS`:

    @import url(https://example.com/styles.css);

#### W kodzie - niezalecane

Możesz też załadować zewnętrzny plik CSS za pomocą właściwości `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Pamiętaj jednak, że jeśli to zrobisz, Twój CSS nie będzie przez nas testowany. 

### Stylowanie modala profilu użytkownika

Modalne okna profili użytkowników również można stylować za pomocą niestandardowego CSS. Jednak aby zapewnić zastosowanie niestandardowego stylowania do profili użytkowników, wszystkie selektory CSS muszą być poprzedzone prefiksem `.user-profile`. Bez tego prefiksu niestandardowe style będą ignorowane dla modalnych okien profilu użytkownika.

Na przykład:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Kompatybilność wsteczna

W FastComments wiemy, że nasi klienci dostosowują widget komentarzy. To jest zamierzone — ostatnią rzeczą, jakiej chcemy, jest powodowanie niezgodności w projekcie Twojego produktu.

Ponieważ jest to ważna część naszego produktu, mamy pipeline budowania, który pozwala nam przeglądać zmiany w widgetcie komentarzy dla każdego klienta przy każdym wydaniu.

Jeśli znajdziemy drobne problemy, zaktualizujemy Twoje konto, aby zapewnić płynność wydania. Jeśli zauważymy poważne zmiany łamiące kompatybilność, pozwala nam to wstrzymać wydanie.

---