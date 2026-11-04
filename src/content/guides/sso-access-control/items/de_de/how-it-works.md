FastComments Zugriffssteuerung funktioniert, indem `Pages` und `Users` den gewünschten Gruppen zugewiesen werden.

Eine Gruppe ist einfach ein String-Kennzeichen, wie `GREEN` oder `abc-123`.

`Users` und `Pages` sind nicht auf nur eine Gruppe beschränkt. Sie sind jeweils auf `100` bzw. `1000` Gruppen begrenzt. 

#### Zugriff auf nicht autorisierte Seiten

Wenn ein Benutzer versucht, auf eine Seite zuzugreifen, auf die er keinen Zugriff hat, sieht er eine Fehlermeldung wie unten:

<div class="screenshot white-bg">
    <div class="title">Beispiel für Autorisierungsfehler</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Beispiel für Autorisierungsfehler" />
</div>

Der Meldungstext kann angepasst werden.