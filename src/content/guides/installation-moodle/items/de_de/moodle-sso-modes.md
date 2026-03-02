Das Plugin unterstützt drei SSO-Modi zur Integration von Moodle-Benutzerkonten in FastComments.

#### Sicheres SSO (Empfohlen)

Benutzerdaten werden serverseitig mit HMAC-SHA256 und Ihrem API Secret signiert. Dies ist die sicherste Option und wird für den Produktionseinsatz empfohlen.

Mit sicherem SSO:

- Benutzernamen, E-Mail-Adressen und Avatare werden sicher an FastComments übermittelt.
- Moodle-Site-Administratoren werden automatisch als FastComments-Moderatoren synchronisiert.
- Benutzer können sich nicht als andere Konten ausgeben.
- Erfordert, dass das **API Secret** in den Plugin-Einstellungen konfiguriert ist.

#### Einfaches SSO

Benutzerdaten (Name, E-Mail, Avatar) werden clientseitig ohne kryptografische Signatur gesendet. Das ist einfacher einzurichten, da kein API Secret erforderlich ist, jedoch weniger sicher, weil Benutzerdaten serverseitig nicht verifiziert werden.

#### Keine

Keine SSO-Integration. Benutzer kommentieren anonym oder müssen sich separat bei FastComments anmelden. Verwenden Sie dies, wenn Sie nicht möchten, dass Moodle-Benutzerkonten mit Kommentaren verknüpft werden.