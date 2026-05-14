#### Przejdź do konfiguracji LTI 1.3

Zaloguj się do FastComments i przejdź do <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">swojej strony konfiguracji LTI 1.3</a>.

If your account doesn't yet have LTI access, you'll see "LTI not enabled for this account" - contact support to enable it on your plan.

#### Wybierz platformę (opcjonalnie)

W sekcji **Generate a Dynamic Registration URL** użyj rozwijanego menu **Platform**, aby poinformować FastComments, z którym LMS się łączysz:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Other LTI 1.3 platform

Możesz też zostawić ustawienie **Automatyczne wykrywanie**. Platforma jest odczytywana z openid-configuration Twojego LMS podczas rejestracji; rozwijane menu tylko ustawia etykietę wyświetlaną dla powstałej konfiguracji.

#### Wygeneruj URL

Kliknij **Generate URL**. FastComments tworzy jednorazowy token rejestracyjny i pokazuje URL wyglądający tak:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Skopiuj go. Ten URL:

- Jest **jednorazowy** - po tym, jak Twój LMS wywoła go pomyślnie, token zostaje użyty.
- Wygasa po **30 minutach**, jeśli nie zostanie użyty.
- Powinien być przechowywany w tajemnicy - każdy, kto ma ten URL, może zarejestrować narzędzie względem Twojego tenanta w ciągu tych 30 minut.

#### Istniejące konfiguracje

Po pomyślnej rejestracji nowa konfiguracja pojawi się w tabeli **Existing Configurations** na tej samej stronie, z informacjami o Platform, Issuer, Client ID i Status. Możesz usuwać konfiguracje z tej tabeli, jeśli kiedykolwiek będziesz potrzebować wyrejestrować usługę.