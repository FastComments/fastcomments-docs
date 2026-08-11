While FastComments Destek ekibi göçlerde yardımcı olabilirken, çoğu işlem destek personelinin müdahalesi olmadan kolayca gerçekleştirilebilir ve izlenebilir.

We natively support importing exports from the following providers:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

By navigating [buradan](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments içe aktarma sayfası, sağlayıcı seçimi ve bir dışa aktarma dosyası için dosya yükleme alanlarıyla'; title='İçe Aktarma Sayfa Formu' app-screenshot-end]

### Monitoring Imports

FastComments, içe ve dışa aktarmaları işlemek için bir iş işleme sistemi kullanır. Sistem işinizi aldığında, işin durumunu içe veya dışa aktarma arayüzünde periyodik olarak raporlayacaktır.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='İçe aktarma sayfası, çalışan bir içe aktarma işi ve iş işleme sistemi tarafından rapor edilen durumu gösteriyor'; title='İçe Aktarma İş Durumu' app-screenshot-end]

Note that the status for Imports and Export are viewable by all administrators in the account.

If your job fails, it will not automatically be restarted. The import will have to be attempted again. If any import or export fails,
our system administrators are automatically notified. If we identify an issue, we'll reach out to you to see if we can help.

### Re-Running The Import

During some migrations, it is necessary to run the import multiple times. For example, it is common to do a first pass
migration for testing, and then run the import again with the latest data before flipping the switch.

Re-importing the same content **will not create duplicates**.

### Data Security and Expiration

Import files are not accessible via outside requests in any way, and import files are deleted from our system as soon as
the import completes.