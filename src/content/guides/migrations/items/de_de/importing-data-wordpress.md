Unser [WordPress-Plugin](https://wordpress.org/plugins/fastcomments/) verfügt über einen leistungsstarken UI-basierten Importmechanismus. Nach der Installation des Plugins
führt es Sie durch die Verknüpfung Ihrer WordPress-Installation mit FastComments und die Übertragung Ihrer vorhandenen Kommentardaten.

**Dies geschieht, ohne dass etwas manuell kopiert oder heruntergeladen werden muss.**

Der Migrationsprozess wird Ihnen während der Migration über die Benutzeroberfläche angezeigt. Die meisten Migrationen dauern nur wenige Minuten.

Der Mechanismus ist so konzipiert, dass er Ihre WordPress-Installation während der Migration nicht übermäßig belastet.

Wenn Sie Ihre Website von WordPress weg verlagern, können Sie stattdessen einen WordPress-XML- oder CSV-Export importieren, anstatt das Plugin zu verwenden. Siehe
[Moving Your Comments to a New Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & Firewalls

Damit das automatisierte WordPress-Setup funktioniert, müssen wir Aufrufe an Ihre WordPress-Installation tätigen.
Firewalls wie Cloudflare können uns blockieren und dazu führen, dass die Integration fehlschlägt. In solchen Fällen können wir Ihnen [eine Reihe von IPs zur Whitelisting für die Integration bereitstellen](https://fastcomments.com/auth/my-account/help).

### Dateneigentum

Im Falle unserer WordPress-Migration werden alle neuen oder aktualisierten Kommentardaten automatisch im Hintergrund mit Ihrer WordPress-Installation synchronisiert.
Das bedeutet, dass die Kommentare zwar von FastComments selbst bereitgestellt werden, um Ihre WordPress-Bereitstellung zu entlasten, wir sie **aber auch** als Backup in Ihrer Datenbank speichern. Dies bedeutet auch, dass Ihre Daten, wenn Sie von FastComments weg wechseln möchten, bereits migriert und auf dem neuesten Stand sind.