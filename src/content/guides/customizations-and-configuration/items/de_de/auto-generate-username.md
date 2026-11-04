Wenn Nutzer kommentieren oder abstimmen und nicht angemeldet sind, werden sie gebeten, ihre E-Mail-Adresse und ihren Benutzernamen anzugeben.

Für manche Websites ist es eine Hürde, wenn Besucher einen eindeutigen Benutzernamen ausdenken müssen, insbesondere auf Mobilgeräten. FastComments kann
für jeden neuen Besucher einen neutralen Benutzernamen generieren und ihn im Benutzernamen-Feld vorbelegen, z. B. `BraveOtter4172`.

Der Besucher kann ihn unverändert lassen oder durch einen Namen seiner Wahl ersetzen.

Dies kann in der Customization-UI über die Einstellung `Generate Usernames Automatically` aktiviert werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='Die Option Generate Usernames Automatically in der Widget-Customization-UI'; title='Generate Usernames Automatically' app-screenshot-end]

#### Verhalten

- Jeder generierte Name ist eindeutig. Er wird gegen bestehende Konten geprüft und für die Browser-Sitzung dieses Besuchers reserviert, sodass zwei Besuchern nicht derselbe Name angeboten wird.
- Der Name wird nur für Besucher generiert, die noch keinen haben. Angemeldete Nutzer, SSO-Nutzer und Besucher, die bereits kommentiert haben, behalten ihren bestehenden Namen.
- Es funktioniert mit oder ohne [anonymes Kommentieren](/guide-customizations-and-configuration.html#allow-anon). Bei deaktiviertem anonymem Kommentieren gibt der Besucher weiterhin seine E-Mail-Adresse ein, muss sich aber keinen Benutzernamen mehr ausdenken.
- Ein wiederkehrender Besucher, der eine bereits verwendete E-Mail-Adresse eingibt, wird mit seinem bestehenden Konto abgeglichen und behält den Namen dieses Kontos.
- Wenn zusätzlich ein [Default Username](/guide-customizations-and-configuration.html#default-username) festgelegt ist, hat der generierte Name Vorrang.