#### Erwähnen von Benutzern in anderen Gruppen

Wenn zwei Benutzer zu zwei verschiedenen Gruppensätzen gehören und sich diese nicht überschneiden, können sie sich nicht gegenseitig `@mention`.

Wenn ein Benutzer manuell ein `@mention` eingibt und seinen Kommentar absendet, bleibt es als Klartext. Der andere Benutzer wird nicht markiert.

#### Verwaltung der Gruppen

`Groups` werden jeweils mit den API-Ressourcen `Pages` und `SSOUsers` definiert.

Die `Pages` API kann aufgerufen werden, um die Menge der Gruppen zu definieren, die Zugriff auf die Seite haben. Standardmäßig haben alle Gruppen und Benutzer, die keiner Gruppe angehören, Zugriff.

Ebenso kann die `SSOUsers` API aufgerufen werden, um die mit jedem Benutzer verknüpften Gruppen zu definieren.

Für beide Ressourcen gibt es keine Einschränkungen, wann die Gruppen gesetzt oder aktualisiert werden können.

Wenn es nur darum geht, zu verhindern, dass Benutzer sich gegenseitig mit `@mention` markieren, müssen die `Pages` nicht berücksichtigt werden.

##### Hinweis!

Das Definieren und Aktualisieren der SSO-Benutzergruppen erfordert nicht die Verwendung der API und kann stattdessen automatisch aktualisiert werden, indem die Gruppen-IDs im SSO-Payload definiert werden, der an das Kommentar-Widget übergeben wird. Bei großen Gruppenlisten wird dies jedoch nicht empfohlen, da der Benutzer dieses Payload bei jedem Seitenaufruf übermitteln müsste.