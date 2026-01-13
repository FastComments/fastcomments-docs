[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Standardmäßig zeigt FastComments in der oberen rechten Ecke des Kommentarbereichs eine Benachrichtigungsglocke an.

Diese Glocke wird rot und zeigt die Anzahl der Benachrichtigungen des Nutzers an. Beispiele für Benachrichtigungen sind:

- Ein Benutzer hat Ihnen geantwortet.
- Ein Benutzer hat in einem Thread geantwortet, in dem Sie kommentiert haben.
- Ein Benutzer hat Ihren Kommentar positiv bewertet.
- Ein Benutzer hat auf einer Seite geantwortet, die Sie abonniert haben.

Die Benachrichtigungsglocke bietet auch eine Möglichkeit, eine gesamte Seite zu abonnieren.

Wir können die Benachrichtigungsglocke jedoch vollständig deaktivieren:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Dies kann auch ohne Code erfolgen. Auf der Seite zur Anpassung des Widgets finden Sie den Abschnitt "Disable Notification Bell".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---