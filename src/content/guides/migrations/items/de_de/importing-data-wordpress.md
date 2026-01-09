---
Unser [WordPress-Plugin](https://wordpress.org/plugins/fastcomments/) verfügt über einen leistungsstarken, UI-basierten Importmechanismus. Bei der Installation des Plugins,
führt es Sie durch das Verknüpfen Ihrer WordPress-Installation mit FastComments und das Kopieren Ihrer vorhandenen Kommentardaten.

**Dies geschieht, ohne dass irgendetwas manuell kopiert oder heruntergeladen werden muss.**

Der Migrationsprozess wird Ihnen während der Migration über die Benutzeroberfläche angezeigt. Die meisten Migrationen dauern nur ein paar Minuten.

Der Mechanismus ist so konzipiert, dass er während der Migration keine übermäßige Belastung für Ihre WordPress-Installation verursacht.

### CloudFlare & Firewalls

Damit die automatisierte WordPress-Einrichtung funktioniert, müssen wir Aufrufe an Ihre WordPress-Installation durchführen.
Firewalls wie Cloudflare können uns blockieren und die Integration zum Scheitern bringen. In solchen Fällen [können wir
Ihnen](https://fastcomments.com/auth/my-account/help) eine Liste von IPs zur Whitelist für die Integration bereitstellen.

### Datenhoheit

Im Fall unserer WordPress-Migration werden alle neuen oder aktualisierten Kommentardaten automatisch im Hintergrund mit Ihrer WordPress-Installation synchronisiert.
Das bedeutet, dass, während die Kommentare von FastComments selbst ausgeliefert werden, um die Belastung Ihrer WordPress-Installation zu reduzieren, wir sie **auch** in Ihrer Datenbank als Sicherung speichern. Das bedeutet auch, dass, wenn Sie sich entscheiden, FastComments nicht mehr zu verwenden, Ihre Daten bereits migriert und auf dem neuesten Stand sind.

---