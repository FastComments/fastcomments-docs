[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments jest zaprojektowany tak, aby można go było dostosowywać. Sam widget komentarzy działa wewnątrz iframe ze względów bezpieczeństwa, więc aby zastosować własne style, musisz wybrać jedną z dwóch metod.

Pierwsza, najprostsza metoda, którą polecamy, to użycie [strony dostosowywania widgetu](https://fastcomments.com/auth/my-account/customize-widget).

Na stronie dostosowywania widgetu, zobacz sekcję „Pokaż zaawansowane opcje”, pod którą znajduje się obszar oznaczony „Custom CSS”:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Edytor niestandardowego CSS pod opcją Pokaż zaawansowane opcje na stronie dostosowywania widgetu'; title='Obszar wprowadzania niestandardowego CSS' app-screenshot-end]

Ta metoda ma kilka zalet:
1. Wprowadzony CSS jest minifikowany przed wysłaniem do użytkownika, a formatowanie pozostaje spójne w interfejsie edycji.
2. Otrzymujesz wszystkie korzyści płynące z UI dostosowywania widgetu, na przykład łatwe dostosowywanie widgetu komentarzy dla różnych witryn.
3. Gdy wprowadzamy zmiany w widgetcie komentarzy, Twoje własne style będą testowane jako część naszego procesu wydawniczego.

Druga metoda polega na określeniu parametru **customCSS** w konfiguracji widgetu, w następujący sposób:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Jednak ma ona *ograniczenia*:
1. Istnieje limit ilości niestandardowego CSS, który może zostać przekazany, zanim nasze serwery odrzucą żądanie, ze względu na rozmiar nagłówków.
2. Musisz zarządzać niestandardowym CSS w swojej infrastrukturze i systemie budowania. Może to być zarówno zaleta, jak i wada.
3. W tym scenariuszu następuje dodatkowy koszt przesyłania niestandardowego CSS **dwukrotnie** przez sieć – najpierw do naszych serwerów, a potem z powrotem w treści iframe. Dla większości rozmiarów ładunku nie jest to zauważalne.
4. Powszechną optymalizacją jest minifikacja CSS w celu zmniejszenia jego rozmiaru w sieci, jednak przy tej metodzie musisz to obsłużyć samodzielnie.
5. Twój niestandardowy CSS nie będzie testowany, gdy wprowadzamy zmiany.

### Zewnętrzne pliki CSS

Możesz nakazać widgetowi pobranie zewnętrznego pliku, używając `@import`!

Zaleca się umieszczenie `@import` w regule dostosowywania. Dzięki temu, jeśli kiedykolwiek będziemy musieli wprowadzić zmianę w widgetcie komentarzy, będziemy mogli użyć naszego automatycznego narzędzia weryfikacji Twojej konfiguracji. Na przykład, utwórz regułę dostosowywania w UI Dostosowywania Widgetu, kliknij `Advanced` i wprowadź w polu `Custom CSS`:

    @import url(https://example.com/styles.css);

#### W kodzie – niezalecane

Możesz także załadować zewnętrzny plik CSS poprzez właściwość `customCSS`:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Jednak pamiętaj, że Twój CSS nie będzie mógł być testowany przez nas, jeśli użyjesz tej metody.

### Stylowanie modala profilu użytkownika

Modale profilu użytkownika można również stylować przy użyciu niestandardowego CSS. Aby zapewnić, że własne style zostaną zastosowane do profili użytkowników, wszystkie selektory CSS muszą być poprzedzone prefiksem `.user-profile`. Bez tego prefiksu własne style będą ignorowane w modalach profilu użytkownika.

Na przykład:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Zgodność wsteczna

W FastComments wiemy, że nasi klienci dostosowują widget komentarzy. To jest zamierzone – ostatnią rzeczą, jaką chcemy, jest spowodowanie niespójności projektowych w Twoim produkcie.

Ponieważ jest to ważna część naszego produktu, mamy pipeline budowania, który pozwala nam przeglądać zmiany w widgetcie komentarzy, per‑klient, przy każdym wydaniu.

Jeśli znajdziemy drobne problemy, zaktualizujemy Twoje konto, aby zapewnić płynne wydanie. Jeśli zauważymy poważne, łamiące zmiany, pozwoli nam to wstrzymać wydanie.