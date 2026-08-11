[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Z FastComments cały tekst w widżecie komentarzy jest konfigurowalny.

Możesz nadpisać pojedynczy fragment tekstu, na przykład przycisk wyślij, lub cały tekst w całym widżecie komentarzy.

Domyślnie tekst w widżecie komentarzy jest tłumaczony w zależności od lokalizacji użytkownika. Jednak możemy nadpisać tekst, jeśli jesteśmy pewni, że nasza baza użytkowników używa tej samej lokalizacji/języka, na przykład:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Niestandardowy tekst'; code-example-end]

Wszystkie konfigurowalne tłumaczenia można znaleźć <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">tutaj</a> pod zakładką „zaawansowane opcje”.

Jednak istnieje prostszy sposób, poprzez interfejs UI dostosowywania widżetu. Tam możemy po prostu znaleźć tekst wyświetlany w widżecie komentarzy w lokalizacji EN_US i określić jego zamiennik.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Panel niestandardowego tekstu z wybranym ciągiem widżetu z listy rozwijanej i polem tekstowym zamiennika'; title='Niestandardowy tekst' app-screenshot-end]

Wszystkie nadpisania tłumaczeń obecnie wpływają na wszystkie lokalizacje.

---