Für die meisten Websites ist der einfachste Weg, Kommentare hinzuzufügen, das Anfügen des Felds `FastComments` an Ihre Inhaltstypen. Gehen Sie zu `Structure > Content types > [type] > Manage fields` und fügen Sie das Feld hinzu.

Jede Entität, die das Feld hat, erhält:

- Einen **Status-Umschalter**, damit Redakteure Kommentare pro Entität ein- oder ausschalten können.
- Einen optionalen **benutzerdefinierten Bezeichner**, damit Sie eine stabile ID verwenden können, die nicht an den Drupal-Entitätspfad gebunden ist.

Der Hauptblock `FastComments Widget` kennt dieses Feld und überspringt Entitäten, die es bereits angefügt haben. So können Sie Kommentare pro Entität mit dem Block mischen, ohne das Widget zweimal auf derselben Seite zu sehen.