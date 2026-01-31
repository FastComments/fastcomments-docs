Oto niektóre często spotykane objawy i typowe rozwiązania. 

### Komunikat "This is a demo"

Ten komunikat pojawia się, gdy skopiowałeś kod widżetu z naszej strony głównej, która używa naszego demo tenant. Aby użyć swojego tenant, skopiuj kod widżetu z [tutaj](https://fastcomments.com/auth/my-account/get-acct-code).

### Błąd "FastComments cannot load on this domain"

FastComments musi wiedzieć, które domeny należą do Ciebie, aby uwierzytelniać żądania związane z Twoim kontem. [Zapoznaj się z naszą dokumentacją](/guide-multiple-sites.html#add-domains-to-account), aby zobaczyć, jak rozwiązać ten błąd (wystarczy dodać dokładny subdomenę + domenę do swojego konta).

Zwróć uwagę, że powinno się to zdarzyć dopiero po zakończeniu okresu próbnego. W trakcie okresu próbnego żądania z nowych domen będą automatycznie dodawane do Twojego konta.

### Przeniesione komentarze nie wyświetlają się w niestandardowych instalacjach

Zazwyczaj dzieje się tak, gdy zaimportowane komentarze są powiązane z `Page ID`, a Ty przekazujesz URL (lub żadnej wartości, wtedy domyślnie używany jest URL strony).

Możesz to zdebugować, [eksportując swoje komentarze](https://fastcomments.com/auth/my-account/manage-data/export) i przeglądając kolumnę `URL ID` (obecnie kolumna `B`).

Upewnij się, że wartości widoczne w kolumnie `URL ID` są tymi samymi wartościami, które przekazujesz do konfiguracji widżetu jako parametr `urlId`.

Aby uzyskać dalsze wyjaśnienia, przeczytaj naszą [dokumentację: Jak komentarze są powiązane ze stronami i artykułami](/guide-customizations-and-configuration.html#url-id).

Jeśli nic nie pomaga, [skontaktuj się z nami](https://fastcomments.com/auth/my-account/help).

### Widżet komentarzy się nie wyświetla

Jeśli widżet komentarzy się nie wyświetla, sprawdź konsolę deweloperską Chrome pod kątem błędów.

W przypadku większości błędnych konfiguracji widżet komentarzy przynajmniej pokaże błąd na stronie, jeśli uda mu się załadować. Brak jakichkolwiek komunikatów zwykle wskazuje na błąd skryptu.

### Żądana konfiguracja nie działa zgodnie z oczekiwaniami

Wypróbuj nasze [rozszerzenie Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US), aby zobaczyć, jaka konfiguracja jest przekazywana do widżetu komentarzy. Jeśli wszystko zawiedzie, zrób zrzut ekranu tego, co pokazuje rozszerzenie Chrome i [skontaktuj się z nami](https://fastcomments.com/auth/my-account/help).

### Brak komentarzy przy tym samym URL z różnymi hashbangami

Domyślnie FastComments używa URL strony jako "bucketu", w którym przechowywane są komentarze. Jeśli Twoje URL zawierają `#hashbangs`, a te `#hashbangs` nie powinny być częścią identyfikatora wątków komentarzy, możemy po prostu zignorować wartość hash banga, na przykład:

[inline-code-attrs-start title = 'Przykład ignorowania hashbangów'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
}];
</script>
[inline-code-end]

Zwróć uwagę, że po wprowadzeniu tej zmiany będzie trzeba wykonać migrację istniejących komentarzy. [W tym celu skontaktuj się z nami.](https://fastcomments.com/auth/my-account/help)

### Parametry zapytania URL wpływające na widżet

Domyślnie FastComments używa URL strony jako "bucketu", w którym przechowywane są komentarze. Jeśli Twoje URL zawierają parametry zapytania, które nie powinny być częścią identyfikatora wątku komentarzy, możemy je po prostu zignorować, na przykład:

[inline-code-attrs-start title = 'Ignoruj parametry zapytania'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
}];
</script>
[inline-code-end]

Zwróć uwagę, że po wprowadzeniu tej zmiany będzie trzeba wykonać migrację istniejących komentarzy. [W tym celu skontaktuj się z nami.](https://fastcomments.com/auth/my-account/help)

### Nie otrzymujesz e-maili

W FastComments wkładamy wiele pracy, aby zapewnić jak największą niezawodność dostarczania e-maili. Jednak niektórzy dostawcy poczty są wyjątkowo trudni do obsługi pod względem dostarczania wiadomości. Sprawdź folder spam dla wiadomości od fastcomments.com.

Jeśli [skontaktujesz się z nami](https://fastcomments.com/auth/my-account/help), zwykle możemy udzielić więcej informacji, dlaczego możesz nie otrzymywać od nas e-maili.