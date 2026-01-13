FastComments otomatik bir bakım modunu destekler. Veritabanı çökerse, popüler yorum dizilerini sunmaya devam edebilir.

Ayrıca bakım modunda tüm yorumlar `BACKUP_DIR`'ye kaydedilir. Sistem tekrar çevrimiçi olduğunda işlenecekler (spam kontrolü vb.) ve kaydedilecekler.

Bunu, her saat en popüler 100 yorum dizisini belirleyip içeriklerini diskte önbelleğe alarak yapar. En popüler 100 dizinin belirlenmesi
önceden hesaplanmış durumdan yapıldığından, ağır bir periyodik iş değildir.

Bu tamamen isteğe bağlıdır ve yalnızca `CACHE_DIR` ve `BACKUP_DIR` ayarlıysa etkinleştirilir. Bu elbette uygulama düğümlerini durumlu hale getirir, ancak bu durum
uygulamanın hatalı davranmasına neden olmadan her zaman kaybolabilir.

Bakım modunda yorum dizilerinin uygun şekilde kimlik doğrulaması yapılamayacağını unutmayın; bu nedenle yalnızca güvenle genel kabul edilen diziler periyodik olarak yedeklenir.

Bakım modunda birçok özellik kullanılamaz.