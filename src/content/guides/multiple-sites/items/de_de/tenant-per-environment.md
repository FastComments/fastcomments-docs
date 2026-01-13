Es ist üblich, für jede Test- oder Entwicklungsumgebung mit FastComments einen Unter-Mandanten zu haben. Jeder Mandant hätte seine eigene Konfiguration, eigene Daten und eigene API-Schlüssel. Konfiguration, Daten und Nutzer können nicht zwischen Mandanten geteilt werden.
Alles ist isoliert. Super-Administratoren des übergeordneten Mandanten können sich jedoch als Benutzer in Unter-Mandanten ausgeben.

There are two approaches:

- The main tenant is for production, and sub-tenants are for test environments.
- The main tenant is simply for billing, and each sub-tenant is for prod, test, and so on.

Die erste Option ist für Benutzer in der Regel leichter nachvollziehbar, aber das kann von Ihrer Organisation abhängen.

Mandanten können [hier](https://eu.fastcomments.com/auth/my-account/tenants) erstellt werden, wenn Sie das Paket haben. Dort würden sich auch die Super-Administratoren
als Benutzer ausgeben. Mandanten können auch über die API für individuellere/automatisierte Setups erstellt werden.

Unabhängig von der gewählten Vorgehensweise müssen Sie die Moderatoren und Benutzer hinzufügen, die Produktionsdaten sehen möchten, in dem "prod"-Mandanten. Wenn Sie sich also
für Option B entscheiden und den übergeordneten Mandanten nur für die Abrechnung nutzen und einen Unter-Mandanten für "prod" anlegen, sollten Sie den Mandanten hinzufügen, zum neuen Mandanten wechseln und Ihre
Admin- und Moderatorbenutzer für den Unter-Mandanten hinzufügen. 

Abschließend zur Klarstellung: Die Seite "Kommentare moderieren" wird bei Option B für den übergeordneten Mandanten leer sein.