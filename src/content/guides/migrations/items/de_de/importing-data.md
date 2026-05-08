Obwohl der FastComments-Support bei Migrationen helfen kann, können die meisten einfach durchgeführt und überwacht werden, ohne dass ein Eingreifen
des Support-Teams erforderlich ist.

Wir unterstützen nativ das Importieren von Exportdateien der folgenden Anbieter:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

By navigating [here](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### Überwachung von Importen

FastComments verwendet ein Jobverarbeitungssystem zur Verarbeitung von Importen und Exporten. Sobald das System Ihren Auftrag übernommen hat, meldet es regelmäßig den Status des Auftrags in der Import- oder Export-Benutzeroberfläche.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

Beachten Sie, dass der Status für Importe und Exporte von allen Administratoren im Account eingesehen werden kann.

Wenn Ihr Auftrag fehlschlägt, wird er nicht automatisch neu gestartet. Der Import muss erneut versucht werden. Wenn ein Import oder Export fehlschlägt, werden unsere Systemadministratoren automatisch benachrichtigt. Wenn wir ein Problem feststellen, melden wir uns bei Ihnen, um zu prüfen, ob wir helfen können.

### Import erneut ausführen

Bei einigen Migrationen ist es notwendig, den Import mehrfach auszuführen. Zum Beispiel ist es üblich, eine erste Migration als Testdurchlauf durchzuführen und den Import dann erneut mit den aktuellsten Daten auszuführen, bevor die Umstellung erfolgt.

Das erneute Importieren desselben Inhalts **erstellt keine Duplikate**.

### Datensicherheit und Löschung

Importdateien sind in keiner Weise über externe Anfragen zugänglich, und Importdateien werden aus unserem System gelöscht, sobald der Import abgeschlossen ist.

---