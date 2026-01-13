Oto niektóre symptomy, które często spotykamy, i typowe rozwiązania.

### Komunikat "This is a demo"

Jest to pokazywane, gdy skopiowałeś kod widgetu z naszej strony głównej, która używa naszego tenanta demo. Aby użyć swojego tenanta, skopiuj kod widgetu [stąd](https://fastcomments.com/auth/my-account/get-acct-code).

### Błąd "FastComments cannot load on this domain"

FastComments musi wiedzieć, które domeny należą do Ciebie, aby uwierzytelniać żądania związane z Twoim kontem. [Sprawdź naszą dokumentację](/guide-multiple-sites.html#add-domains-to-account), aby zobaczyć, jak rozwiązać ten błąd (po prostu dodaj dokładną subdomenę + domenę do swojego konta).

Należy pamiętać, że powinno to wystąpić dopiero po zakończeniu okresu próbnego. W okresie próbnym wszelkie żądania z nowych domen zostaną automatycznie dodane do Twojego konta.

### Zmigrowane komentarze nie pokazują się dla niestandardowych instalacji

Zwykle dzieje się tak, gdy zaimportowane komentarze są powiązane z `Page ID`, a Ty przekazujesz URL (lub brak wartości, w którym to przypadku domyślnie używany jest URL strony).

Możesz to debugować, [eksportując swoje komentarze](https://fastcomments.com/auth/my-account/manage-data/export) i przeglądając kolumnę `URL ID` (obecnie Kolumna `B`).

Upewnij się, że wartości widoczne w kolumnie `URL ID` są tymi samymi wartościami, które przekazujesz do konfiguracji widgetu jako parametr `urlId`.

Aby uzyskać dalsze wyjaśnienia, spróbuj przeczytać naszą [dokumentację o tym, jak komentarze są powiązane ze stronami i artykułami](/guide-customizations-and-configuration.html#url-id).

Jeśli wszystko inne zawiedzie, [skontaktuj się z nami](https://fastcomments.com/auth/my-account/help).

### Widget komentarzy się nie pokazuje

Jeśli widget komentarzy się nie pokazuje, sprawdź konsolę deweloperską Chrome pod kątem błędów.

Przy większości błędnych konfiguracji widget komentarzy przynajmniej pokaże błąd na stronie, jeśli jest w stanie się załadować. Niewidzenie niczego jest zwykle oznaką błędu skryptowego.

### Pożądana konfiguracja nie działa zgodnie z oczekiwaniami

Wypróbuj nasze [rozszerzenie Chrome](https://chrome.google.com/webstore/detail/fastcomments-debugger/cadggdemhfkjjghkdbfhonoccnplffjj?hl=en-US), aby zobaczyć, jaka konfiguracja jest przekazywana do widgetu komentarzy. Jeśli wszystko zawiedzie, zrób zrzut ekranu tego, co pokazuje rozszerzenie Chrome i [skontaktuj się z nami](https://fastcomments.com/auth/my-account/help).

### Brakujące komentarze na tym samym URL z innym hash bangiem

Domyślnie FastComments użyje URL strony jako "bucket", gdzie przechowywane są komentarze. Jeśli Twoje URL-e zawierają `#hashbangi`, a te `#hashbangi` nie powinny być częścią identyfikatora identyfikującego wątek komentarzy, możemy po prostu zignorować wartość hash bang, na przykład:

[inline-code-attrs-start title = 'Ignore Hash Bangs Example'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.href.replace(location.hash, ''),
    urlId: location.href.replace(location.hash, '')
});
</script>
[inline-code-end]

Należy pamiętać, że po dokonaniu tej zmiany będzie trzeba przeprowadzić migrację dla istniejących komentarzy. [W tym celu skontaktuj się z nami.](https://fastcomments.com/auth/my-account/help)

### Parametry zapytania URL wpływają na widget

Domyślnie FastComments użyje URL strony jako "bucket", gdzie przechowywane są komentarze. Jeśli Twoje URL-e zawierają parametry zapytania, które nie powinny być częścią identyfikatora identyfikującego wątek komentarzy, możemy po prostu je zignorować, na przykład:

[inline-code-attrs-start title = 'Ignore Query Parameters'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: "demo",
    url: location.protocol + '//' + location.host + location.pathname,
    urlId: location.pathname
});
</script>
[inline-code-end]

Należy pamiętać, że po dokonaniu tej zmiany będzie trzeba przeprowadzić migrację dla istniejących komentarzy. [W tym celu skontaktuj się z nami.](https://fastcomments.com/auth/my-account/help)

### Nie otrzymujesz emaili

W FastComments wkładamy dużo pracy w zapewnienie, że nasza dostawę emaili jest jak najbardziej niezawodna. Jednak niektórzy dostawcy poczty elektronicznej są notorycznie trudni do niezawodnego dotarcia. Sprawdź folder spam w poszukiwaniu wiadomości od fastcomments.com.

Jeśli [skontaktujesz się z nami](https://fastcomments.com/auth/my-account/help), zwykle możemy zapewnić więcej informacji o tym, dlaczego możesz nie widzieć emaili od nas.
