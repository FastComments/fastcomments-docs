Wenn von FastComments gesendete E-Mails zurückkommen (Bounce) oder vom Empfänger als Spam markiert werden, fügt der E-Mail-Anbieter diese Adresse einer Unterdrückungsliste hinzu. FastComments synchronisiert diese Unterdrückungslisten täglich, sodass keine weiteren E-Mails an Adressen gesendet werden, die sie nicht empfangen können.

Benutzer und Moderatoren mit unterdrückten E-Mail-Adressen erhalten keine E-Mail-Benachrichtigungen, einschließlich Antwortbenachrichtigungen, Erwähnungsbenachrichtigungen, Admin-Benachrichtigungen und Digest-E-Mails. Ein rotes "E-Mail unterdrückt"-Abzeichen erscheint neben betroffenen Benutzern und Moderatoren in der Admin-Benutzeroberfläche.

#### Unterdrückte E-Mails anzeigen

Mandantenadministratoren mit der Berechtigung "Daten verwalten" können unterdrückte E-Mails auf der
[Unterdrückte E-Mails](https://fastcomments.com/auth/my-account/suppressed-emails) Seite unter "Daten verwalten" einsehen.

Die Seite zeigt eine Tabelle aller unterdrückten E-Mail-Adressen, die mit den Benutzern, Moderatoren und Kommentatoren Ihres Mandanten verknüpft sind. Sie können nach E-Mail-Adresse über das Suchfeld filtern.

#### Unterdrückung entfernen

Um eine Unterdrückung zu entfernen, klicken Sie auf die **Entfernen**-Schaltfläche neben dem Eintrag in der Tabelle. Sie werden zu einer Bestätigungsseite weitergeleitet, die die Details der Unterdrückung anzeigt. Klicken Sie auf **Entfernung bestätigen**, um fortzufahren.

Wenn eine Unterdrückung entfernt wird, kontaktiert FastComments den E-Mail-Anbieter, um die Adresse zu entsperren, und entfernt das Unterdrückungskennzeichen aus allen zugehörigen Benutzer- und Moderator-Datensätzen.

#### Ratenbegrenzungen

Um Missbrauch zu verhindern, sind Entfernungen in ihrer Häufigkeit begrenzt:

- Jede E-Mail-Adresse kann nur einmal alle 30 Tage wieder freigeschaltet werden.
- Jeder Mandant kann bis zu 5 Entfernungen pro Kalendermonat durchführen.

Wenn eine Unterdrückung nach der Entfernung erneut auftritt, bedeutet dies, dass die E-Mail-Adresse erneut gebounced ist oder wieder als Spam gemeldet wurde. In diesem Fall sollte das zugrunde liegende Zustellbarkeitsproblem behoben werden, bevor ein weiterer Entfernungsversuch unternommen wird.