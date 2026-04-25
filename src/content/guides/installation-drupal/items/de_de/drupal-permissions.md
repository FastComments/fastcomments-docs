Das Modul fügt drei Drupal-Berechtigungen hinzu, die Sie pro Rolle unter `People > Permissions` zuweisen können.

- **Administer FastComments** - Zugriff auf das FastComments-Einstellungsformular unter `/admin/config/content/fastcomments`.
- **View FastComments** - Erforderlich, um das Kommentierungs-Widget zu sehen. Ohne diese Berechtigung wird das Widget nicht gerendert.
- **Toggle FastComments** - Ermöglicht es Benutzern, Kommentare pro Entität mithilfe des Feld-Widgets zu aktivieren oder zu deaktivieren.

Standardmäßig können nur Benutzer mit der Berechtigung `administer site configuration` die FastComments-Einstellungen ändern. Weisen Sie `View FastComments` anonymen und authentifizierten Benutzern zu, wenn Besucher das Widget sehen sollen.