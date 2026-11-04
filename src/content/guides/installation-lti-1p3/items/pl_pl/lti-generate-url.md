#### Przejdź do konfiguracji LTI 1.3

Zaloguj się do FastComments i przejdź do <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">strony konfiguracji LTI 1.3</a>.

Jeśli Twoje konto nie ma jeszcze dostępu do LTI, zobaczysz komunikat "LTI not enabled for this account" - skontaktuj się z pomocą techniczną, aby włączyć tę funkcję w Twoim planie.

#### Wybierz platformę (opcjonalnie)

W sekcji **Generate a Dynamic Registration URL** użyj listy rozwijanej **Platform**, aby poinformować FastComments, z którym systemem LMS się łączysz:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Inna platforma LTI 1.3

Możesz też pozostawić opcję **Auto-detect**. Platforma jest odczytywana z konfiguracji openid Twojego LMS podczas rejestracji; lista rozwijana służy jedynie do ustawienia etykiety wyświetlanej dla powstałej konfiguracji.

#### Wygeneruj adres URL

Kliknij **Generate URL**. FastComments utworzy jednorazowy token rejestracyjny i wyświetli adres URL wyglądający tak:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Skopiuj go. Ten adres URL:

- Jest **jednorazowy** - gdy Twój LMS pomyślnie z niego skorzysta, token zostaje zużyty.
- Wygasa po **30 minutach**, jeśli nie zostanie użyty.
- Powinien być przechowywany w tajemnicy - każdy, kto zna ten adres URL, może zarejestrować narzędzie w Twojej dzierżawie w ciągu tych 30 minut.

#### Istniejące konfiguracje

Po pomyślnym zakończeniu rejestracji nowa konfiguracja pojawi się w tabeli **Existing Configurations** na tej samej stronie, wraz z informacjami o platformie, issuerze, identyfikatorze klienta (Client ID) i statusie. W razie potrzeby możesz usunąć konfiguracje z tej tabeli, aby wyrejestrować narzędzie.