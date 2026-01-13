Standardmäßig synchronisiert FastComments täglich zurück auf Ihre WordPress-Website. Dies dient ausschließlich zu Sicherungszwecken, damit Sie weiterhin eine Kopie der Daten besitzen, und für Plugins, die davon abhängig sein könnten.

Dies geschieht nicht sofort bei jedem gespeicherten Kommentar, da einige Websites zwar hohen Leseverkehr bewältigen können, ihre Datenbankbereitstellungen jedoch nicht immer in der Lage sind, den hohen Schreibverkehr zu bewältigen (daher wird diese Arbeit an FastComments ausgelagert).

Der Synchronisationsplan zurück zu WordPress kann durch Installation eines Plugins angepasst werden. Wir empfehlen [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description).

Steps:

1. Install WP Crontrol
2. Go to `Settings -> Cron Schedules`.
3. Go to the `Cron Events` tab.
4. Search for `fastcomments_cron_hook`.
5. Edit the event. You can configure the hook to run hourly, twice a day, daily (default), or once a week.

Die Synchronisierung zurück nach WordPress kann auch jederzeit manuell durchgeführt werden, indem Sie zum FastComments-Plugin-Dashboard gehen und `Manually Sync` auswählen. Sie haben die Möglichkeit, zurück zu Ihrer WP-Installation zu synchronisieren oder Ihre WP-Kommentare erneut auf die FastComments-Server hochzuladen.