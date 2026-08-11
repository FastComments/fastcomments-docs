[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Mit FastComments ist aller Text im Kommentar-Widget anpassbar.

Sie können ein einzelnes Textelement überschreiben, z. B. die Senden‑Schaltfläche, oder den gesamten Text im gesamten Kommentar‑Widget.

Standardmäßig wird der Text im Kommentar-Widget basierend auf der Locale des Benutzers übersetzt. Wir können den Text jedoch überschreiben, wenn wir sicher sind, dass unsere Benutzerbasis dieselbe Locale/Sprache verwendet, zum Beispiel:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Benutzerdefinierter Text'; code-example-end]

Alle anpassbaren Übersetzungen finden Sie <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">hier</a> unter dem Tab „Erweiterte Optionen“.

Es gibt jedoch einen einfacheren Weg über die Widget‑Anpassungs‑UI. Dort können wir einfach den Text finden, der im Kommentar‑Widget in der EN_US‑Locale angezeigt wird, und eine
Ersetzung angeben.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Benutzerdefiniertes Textfeld mit einer aus dem Dropdown ausgewählten Widget‑Zeichenkette und einem Ersetzungstextfeld'; title='Benutzerdefinierter Text' app-screenshot-end]

Alle Überschreibungen von Übersetzungen wirken derzeit auf alle Locales.

---