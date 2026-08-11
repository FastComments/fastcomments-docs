[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Standardmäßig zeigt FastComments eine Benachrichtigungsglocke oben rechts im Kommentarbereich.

Diese Glocke wird rot und zeigt eine Anzahl der Benachrichtigungen, die der Benutzer hat. Einige Beispielbenachrichtigungen sind:

- Benutzer hat Ihnen geantwortet.
- Benutzer hat in einem Thread geantwortet, in dem Sie kommentiert haben.
- Benutzer hat Ihren Kommentar positiv bewertet.
- Benutzer hat auf einer Seite geantwortet, die Sie abonniert haben.

Die Benachrichtigungsglocke bietet zudem einen Mechanismus, um eine gesamte Seite zu abonnieren, als auch.

Allerdings können wir die Benachrichtigungsglocke vollständig deaktivieren:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Dies kann auch ohne Code durchgeführt werden. Auf der Widget‑Anpassungsseite finden Sie den Abschnitt „Benachrichtigungsglocke deaktivieren“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Widget-Anpassungsseite mit dem Kontrollkästchen „Benachrichtigungsglocke deaktivieren“ aktiviert'; title='Benachrichtigungsglocke deaktivieren' app-screenshot-end]