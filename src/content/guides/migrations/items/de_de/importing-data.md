Während FastComments Support bei Migrationen helfen kann, können die meisten davon leicht durchgeführt und überwacht werden, ohne dass Support-Mitarbeiter eingreifen müssen.

Wir unterstützen nativ das Importieren von Exporten der folgenden Anbieter:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (über das Plugin)

Wenn Sie [hier](https://fastcomments.com/auth/my-account/manage-data/import) navigieren, können wir die Datei hochladen, die die zu migrierenden Daten enthält.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Monitoring Imports

FastComments verwendet ein Job-Processing-System zum Verarbeiten von Importen und Exporten. Sobald das System Ihren Job übernommen hat, wird es
periodisch den Status des Jobs in der Import- oder Export-Benutzeroberfläche melden.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Beachten Sie, dass der Status von Importen und Exporten für alle Administratoren im Konto sichtbar ist.

Wenn Ihr Job fehlschlägt, wird er nicht automatisch neu gestartet. Der Import muss erneut versucht werden. Wenn ein Import oder Export fehlschlägt,
werden unsere Systemadministratoren automatisch benachrichtigt. Wenn wir ein Problem identifizieren, werden wir uns mit Ihnen in Verbindung setzen, um zu sehen, ob wir helfen können.

### Re-Running The Import

Während einiger Migrationen ist es notwendig, den Import mehrfach auszuführen. Zum Beispiel ist es üblich, einen ersten Durchlauf
zur Prüfung durchzuführen und den Import dann erneut mit den neuesten Daten laufen zu lassen, bevor man den Schalter umlegt.

Das erneute Importieren desselben Inhalts **erstellt keine Duplikate**.

### Data Security and Expiration

Importdateien sind in keiner Weise über externe Anfragen zugänglich, und Importdateien werden aus unserem System gelöscht, sobald
der Import abgeschlossen ist.