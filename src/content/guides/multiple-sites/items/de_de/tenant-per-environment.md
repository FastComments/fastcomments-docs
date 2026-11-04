Es ist üblich, bei FastComments einen Sub-Tenant pro Test- oder Dev-Umgebung zu haben. Jeder Tenant hätte seine eigene Konfiguration, seine eigenen Daten und API-Schlüssel. Konfiguration, Daten und Benutzer können nicht über Tenants hinweg geteilt werden.
Alles ist isoliert. Super-Admins des übergeordneten Tenants können sich jedoch als Benutzer in Child-Tenants ausgeben.

Es gibt zwei Ansätze:

- Der Haupt-Tenant ist für die Produktion, und Sub-Tenants sind für Testumgebungen.
- Der Haupt-Tenant dient lediglich der Abrechnung, und jeder Sub-Tenant ist für Prod, Test und so weiter.

Der erste Ansatz ist für Benutzer im Allgemeinen einfacher zu verstehen, aber dies kann von Ihrer Organisation abhängen.

Tenants können [hier](https://eu.fastcomments.com/auth/my-account/tenants) erstellt werden, wenn Sie das Paket haben. Hier würden auch Super-Admins
die Identität von Benutzern übernehmen. Tenants können auch über die API erstellt werden, für benutzerdefiniertere/automatisierte Setups.

Unabhängig vom gewählten Ansatz müssen Sie die Moderatoren und Benutzer hinzufügen, die Produktionsdaten im "prod"-Tenant sehen möchten. Wenn Sie sich also zum Beispiel
für Option B entscheiden und den übergeordneten Tenant für die Abrechnung nutzen und einen Sub-Tenant für "prod" haben möchten, müssen Sie den Tenant hinzufügen, zum neuen Tenant wechseln und Ihre
Admin- und Moderator-Benutzer für den Sub-Tenant hinzufügen.

Zur Klarstellung: Die Seite "Kommentare moderieren" ist bei Option B für den übergeordneten Tenant leer.