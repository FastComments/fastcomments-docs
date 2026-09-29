When users comment or vote, and they are not logged in, they will be asked to provide their email and username.

For some sites, asking a visitor to invent a unique username is a hurdle, particularly on mobile. FastComments can
generate a neutral username for each new visitor and prefill it in the username field, like `BraveOtter4172`.

The visitor can leave it as-is, or replace it with a name of their choosing.

This can be enabled from the Customization UI, under the setting called `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='Die Option \'Benutzernamen automatisch generieren\' in der Widget-Anpassungsoberfläche'; title='Benutzernamen automatisch generieren' app-screenshot-end]

#### Wie es funktioniert

- Jeder generierte Name ist eindeutig. Er wird mit bestehenden Konten abgeglichen und für die Browsersitzung dieses Besuchers reserviert, sodass zwei Besucher nicht denselben Namen angeboten bekommen.
- Der Name wird nur für Besucher generiert, die noch keinen haben. Eingeloggte Benutzer, SSO‑Benutzer und Besucher, die bereits kommentiert haben, behalten ihren bestehenden Namen.
- Es funktioniert mit oder ohne [anonymes Kommentieren](/guide-customizations-and-configuration.html#allow-anon). Wenn anonymes Kommentieren deaktiviert ist, gibt der Besucher weiterhin seine E‑Mail-Adresse ein, muss aber keinen Benutzernamen mehr überlegen.
- Ein wiederkehrender Besucher, der eine bereits zuvor verwendete E‑Mail-Adresse eingibt, wird seinem bestehenden Konto zugeordnet und behält den Namen dieses Kontos.
- Wenn ein [Standardbenutzername](/guide-customizations-and-configuration.html#default-username) ebenfalls festgelegt ist, hat der generierte Name Vorrang.