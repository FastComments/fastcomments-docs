Mit FastComments SSO Access Control, manchmal als RBAC bezeichnet, können Benutzer darauf beschränkt werden, nur auf bestimmte Seiten oder Kommentar-Threads zuzugreifen. Zusätzlich,
können sich Benutzer nur gegenseitig `@mention` in derselben Gruppe.

## Im Detail

Wir können `Users` und optional `Pages` in Gruppen einteilen.

Wenn `Users` in Gruppen eingeordnet werden und `Limit Comments by SSO User Groups` in den Widget-Einstellungen aktiviert ist, dann sehen Benutzer nur Kommentare von Benutzern in ihren eigenen Gruppen und können nur Benutzer in denselben Gruppen `@mention`.

Außerdem können `Pages` in Gruppen eingeordnet werden, und dann können Benutzer nur auf Kommentare von Seiten zugreifen, auf die sie Zugriff haben.

Wir nennen das "User-Level"-Gruppen im Gegensatz zu "Page-Level"-Gruppen.

Welche ist also die richtige für Sie?

#### Verwenden Sie User-Level-Gruppen, wenn...

- Sie denselben `urlId`-Wert (Seiten-URL oder Artikel-ID) verwenden möchten, aber Kommentare nach Gruppe einschränken wollen.
- Zum Beispiel möchten Sie Gruppen "New User" und "Veteran User" haben, und sie sollten niemals die Kommentare der jeweils anderen auf denselben Seiten sehen.

#### Verwenden Sie Page-Level-Gruppen, wenn...

- Ihre Gruppen spezifische Seiten haben.
- Zum Beispiel sollten Benutzer in der Gruppe "Public Pages" niemals Artikel der Gruppe "Top Secret" ansehen.

---