Our [WordPress‑Plugin](https://wordpress.org/plugins/fastcomments/) hat einen leistungsstarken UI‑basierten Import‑Mechanismus. Nach der Installation des Plugins führt es Sie durch die Verknüpfung Ihrer WordPress‑Installation mit FastComments und das Kopieren Ihrer bestehenden Kommentardaten.

**Dies geschieht, ohne dass Sie manuell etwas kopieren oder herunterladen müssen.**

Der Migrationsvorgang wird Ihnen über die UI während der Migration angezeigt. Die meisten Migrationen dauern nur ein paar Minuten.

Der Mechanismus ist so konzipiert, dass er während der Migration keine übermäßige Belastung Ihrer WordPress‑Installation verursacht.

Wenn Sie Ihre Seite von WordPress weg verlegen, können Sie stattdessen einen WordPress‑XML‑ oder CSV‑Export importieren, anstatt das Plugin zu verwenden. Siehe [Verschieben Ihrer Kommentare zu einer neuen Seite](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & Firewalls

Damit die automatisierte WordPress‑Einrichtung funktioniert, müssen wir Aufrufe an Ihre WordPress‑Installation tätigen. Firewalls wie Cloudflare können uns blockieren und dazu führen, dass die Integration fehlschlägt. In solchen Fällen können [wir Ihnen](https://fastcomments.com/auth/my-account/help) ein Set von IP‑Adressen zur Whitelist‑Eintragung für die Integration bereitstellen.

### Datenbesitz

Im Falle unserer WordPress‑Migration werden alle neuen oder aktualisierten Kommentardaten automatisch im Hintergrund zurück zu Ihrer WordPress‑Installation synchronisiert. Das bedeutet, dass, während die Kommentare von FastComments selbst bereitgestellt werden, um die Last von Ihrer WordPress‑Installation zu nehmen, wir **auch** eine Sicherungskopie in Ihrer Datenbank speichern. Das bedeutet außerdem, dass Sie, wenn Sie von FastComments wegwechseln möchten, Ihre Daten bereits migriert und aktuell sind.