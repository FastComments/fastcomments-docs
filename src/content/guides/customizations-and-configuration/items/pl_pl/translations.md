[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

W FastComments cały tekst w widżecie komentarzy jest konfigurowalny.

Możesz nadpisać pojedynczy element tekstu, na przykład przycisk wysyłania, lub cały tekst w całym widżecie komentarzy.

Domyślnie tekst w widżecie komentarzy jest tłumaczony na podstawie ustawień regionalnych użytkownika. Możemy jednak nadpisać tekst, jeśli jesteśmy pewni
że nasza baza użytkowników używa tego samego regionu/języka, na przykład:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Wszystkie konfigurowalne tłumaczenia można znaleźć <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">tutaj</a> w zakładce "zaawansowane opcje".

Istnieje jednak prostszy sposób, przez interfejs dostosowywania widżetu. Tam możemy po prostu znaleźć tekst, który pokazuje się w widżecie komentarzy w lokalizacji EN_US i określić
zamiennik.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Wszystkie nadpisania tłumaczeń obecnie dotyczą wszystkich lokalizacji.