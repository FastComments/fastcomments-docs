Standardmäßig synchronisiert FastComments täglich zurück zu Ihrer WordPress-Seite. Dies dient ausschließlich Backup-Zwecken, damit Sie weiterhin eine Kopie der Daten besitzen, sowie für Plugins, die davon abhängig sein könnten.

Dies geschieht nicht sofort bei jedem gespeicherten Kommentar, da einige Websites zwar viel Lese-Traffic bewältigen können, ihre Datenbank-Deployments aber nicht immer in der Lage sind, den hohen Schreib-Traffic zu verarbeiten (daher die Auslagerung dieser Arbeit an FastComments).

Der Synchronisierungszeitplan zurück zu WordPress kann durch die Installation eines Plugins angepasst werden. Wir empfehlen [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Schritte:

1. Installieren Sie WP Crontrol
2. Gehen Sie zu `Settings -> Cron Schedules`.
3. Öffnen Sie den Tab `Cron Events`.
4. Suchen Sie nach `fastcomments_cron_hook`.
5. Bearbeiten Sie das Ereignis. Sie können den Hook so konfigurieren, dass er stündlich, zweimal täglich, täglich (Standard) oder einmal pro Woche ausgeführt wird.

Die Synchronisierung zurück zu WordPress kann jederzeit auch manuell durchgeführt werden, indem Sie zum FastComments-Plugin-Dashboard gehen und `Manually Sync` auswählen. Sie haben die Möglichkeit, zurück zu Ihrer WP-Installation zu synchronisieren oder Ihre WP-Kommentare erneut auf die FastComments-Server hochzuladen.