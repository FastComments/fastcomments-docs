[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Mit FastComments ist sämtlicher Text im Kommentar-Widget anpassbar.

Sie können einen einzelnen Textbaustein überschreiben, z. B. die Absenden-Schaltfläche, oder den gesamten Text des Kommentar-Widgets.

Standardmäßig wird der Text im Kommentar-Widget basierend auf der Locale des Nutzers übersetzt. Wir können den Text jedoch überschreiben, wenn wir sicher sind,
dass unsere Benutzerbasis dieselbe Locale/Sprache verwendet, zum Beispiel:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Alle anpassbaren Übersetzungen finden Sie <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">hier</a> unter der "erweiterten Optionen" Registerkarte.

Es gibt jedoch einen einfacheren Weg über die Widget-Anpassungsoberfläche. Dort können wir einfach den Text finden, der im Kommentar-Widget in der EN_US-Locale angezeigt wird, und einen Ersatz angeben.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Alle Überschreibungen von Übersetzungen wirken sich derzeit auf alle Locales aus.