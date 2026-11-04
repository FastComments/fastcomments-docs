Während das FastComments-Support-Team bei Migrationen helfen kann, lassen sich die meisten Migrationen einfach durchführen und überwachen, ganz ohne Unterstützung
des Support-Personals.

Wir unterstützen nativ den Import von Exporten der folgenden Anbieter:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (über das Plugin oder einen XML- oder CSV-Export)
- AnyComment (über WordPress Import/Export)

Über [diesen Link](https://fastcomments.com/auth/my-account/manage-data/import) können wir die Datei mit den zu migrierenden Daten hochladen.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments-Importseite mit den Feldern zur Anbieterauswahl und zum Hochladen einer Exportdatei'; title='Das Formular der Importseite' app-screenshot-end]

### Importe überwachen

FastComments verwendet ein Job-Verarbeitungssystem für die Verarbeitung von Importen und Exporten. Sobald das System Ihren Job aufgenommen hat, wird der
Status des Jobs regelmäßig in der Import- bzw. Export-Benutzeroberfläche gemeldet.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='Importseite mit einem laufenden Import-Job und dem vom Job-Verarbeitungssystem gemeldeten Status'; title='Status des Import-Jobs' app-screenshot-end]

Beachten Sie, dass der Status von Importen und Exporten von allen Administratoren im Konto eingesehen werden kann.

Wenn Ihr Job fehlschlägt, wird er nicht automatisch neu gestartet. Der Import muss erneut versucht werden. Wenn ein Import oder Export fehlschlägt,
werden unsere Systemadministratoren automatisch benachrichtigt. Wenn wir ein Problem identifizieren, melden wir uns bei Ihnen, um zu sehen, ob wir helfen können.

### Import erneut ausführen

Bei manchen Migrationen ist es notwendig, den Import mehrfach auszuführen. Beispielsweise ist es üblich, zunächst einen Testdurchlauf
der Migration durchzuführen und den Import dann mit den neuesten Daten erneut auszuführen, bevor umgeschaltet wird.

Das erneute Importieren desselben Inhalts **erzeugt keine Duplikate**.

### Datensicherheit und Ablauf

Importdateien sind über externe Anfragen in keiner Weise zugänglich, und Importdateien werden aus unserem System gelöscht, sobald
der Import abgeschlossen ist.