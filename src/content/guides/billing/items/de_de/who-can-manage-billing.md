Die Abrechnung ist auf zwei Arten von Benutzern beschränkt:

- Den `Account Owner`, den Benutzer, der das Konto erstellt hat.
- Jeden Benutzer mit der Berechtigung `Billing Admin`.

Sie sind die einzigen Benutzer, die den Plan ändern können, eine Karte speichern oder ersetzen können, `Invoice Settings` öffnen können, Flex-Ausgabenlimits festlegen und Guthaben hinzufügen können. Alle anderen, die es versuchen, sehen `Only the account owner or a billing admin can change the plan or payment method.`

Abrechnungs-E-Mails, wie die Erinnerung in der Mitte des Testzeitraums, Warnungen bei fehlgeschlagenen Zahlungen und Deaktivierungshinweise, werden an Benutzer mit der Berechtigung `Billing Admin` gesendet. Geben Sie die Berechtigung demjenigen, der sie erhalten soll, zum Beispiel jemandem in der Finanzabteilung, und er erhält die E-Mails, ohne dass er sonstigen Zugriff auf das Dashboard benötigt. Die Erinnerungen, die nach einem Testzeitraum ohne Abrechnungsdaten gesendet werden, gehen an jeden Dashboard-Benutzer des Kontos. Jede dieser E-Mails enthält einen Abmeldelink; ein Benutzer, der sich von Konto-Benachrichtigungen abmeldet, erhält sie nicht mehr.

Die Rechnungen selbst gehen an die `Billing Email` in den [Invoice Settings](#invoice-settings), die überhaupt nicht zu einem Dashboard-Benutzer gehören muss.

Die Seite [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) benötigt stattdessen die Berechtigung `Analytics Admin`, da sie die Nutzung und nicht Zahlungsdetails anzeigt.

Um die Berechtigung zu gewähren oder zu entfernen, öffnen Sie die Seite `Users` und bearbeiten Sie den Benutzer. Siehe [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions). Beachten Sie bei Flex, dass jeder zusätzliche Administrator ein abgerechneter Sitzplatz ist, siehe [Admin and Moderator Seats](#admin-and-moderator-seats).

Wenn Sie angemeldet sind und `Billing Info` überhaupt nicht sehen können oder es Sie zur Startseite zurückleitet, sind Sie als Kommentator-Konto und nicht als Dashboard-Benutzer angemeldet. Melden Sie sich ab und mit dem Konto an, dem die Website gehört.