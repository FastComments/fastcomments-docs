FastComments bietet eine automatisierte Möglichkeit, Ihre Kommentare über Domains hinweg zu migrieren.

Die Domain-Migration erfordert lediglich eine `from`- und eine `to`-Domain.

Dies **verschiebt** Kommentare, es kopiert sie nicht. Wenn Sie Kommentare kopieren möchten, kontaktieren Sie uns.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Domain-Migration-Tool mit den Feldern für die Ausgangs- und Ziel-Domain sowie der Anzahl der migrierten Kommentare'; title='Domain-Migration' app-screenshot-end]

Dies ist zum Beispiel auch nützlich, wenn ein Teil Ihrer Migration zu FastComments das Migrieren von einem anderen Anbieter beinhaltet, sodass Ihre Kommentar-Importdaten Daten enthalten können, die migriert werden müssen. In diesem Fall können Sie den Import ausführen und anschließend die Domain-Migration.

### Fortschritt überwachen

Das Domain-Migration-Tool verwendet dasselbe FastComments-Jobverarbeitungssystem wie die anderen Datenverwaltungstools.

Es kann zu einer Verzögerung kommen, bevor Ihre Migration startet. Das ist normal, da das System periodisch nach neuen zu verarbeitenden Jobs sucht.

Während der Job läuft, wird er die Anzahl der zu migrierenden Kommentare sowie die bisher migrierten Kommentare anzeigen.

### Migration von Domains über die API

Domains können auch mit [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post) migriert werden, wobei `type` auf `domain` gesetzt wird. Der Fortschritt ist über [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id) verfügbar, und die Migration kann für 90 Tage rückgängig gemacht werden.

---