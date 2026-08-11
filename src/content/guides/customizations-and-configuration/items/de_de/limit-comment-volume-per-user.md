---
Standardmäßig kann jeder Benutzer bis zu `5 Kommentare` in derselben Minute senden.

Dies wird anhand der Benutzer-ID, der anonymen Benutzer-ID und der IP-Adresse (gehasht) verfolgt.

Dies kann ohne Code auf der Widget-Anpassungsseite angepasst werden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Maximale Kommentare pro Minute Feld auf der Widget-Anpassungsseite, standardmäßig auf 5 gesetzt'; title='Begrenzung des Kommentarvolumens pro Benutzer' app-screenshot-end]

Beachten Sie, dass Sie, wenn Sie die Kommentar-Erstellungs-API verwenden, möglicherweise die ursprüngliche `ip`-Adresse des Benutzers in der Anfrage an unser Backend übergeben möchten, damit die Ratenbegrenzung angewendet wird
pro Benutzer und nicht global für Ihr Konto.

---