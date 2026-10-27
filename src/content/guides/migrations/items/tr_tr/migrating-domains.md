FastComments, yorumlarınızı alanlar arasında taşımanız için otomatik bir yol sağlar.

Alan taşıma işlemi sadece bir `from` ve bir `to` alanı gerektirir.

Bu **yorumları taşır**, kopyalamaz. Yorumları kopyalamak isterseniz, bizimle iletişime geçin.

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='Alan taşıma aracı, from ve to alanları ve taşınan yorum sayısı ile birlikte'; title='Alanları Taşıma' app-screenshot-end]

Bu, örneğin FastComments'a geçişinizin bir kısmı farklı bir sağlayıcıdan taşıma içeriyorsa, yorum içe aktarma verilerinizin taşınması gereken verileri içerebileceği durumlarda da faydalıdır. Bu durumda, içe aktarmayı çalıştırabilir ve ardından alan taşımasını yapabilirsiniz.

### İlerlemeyi İzleme

Alan Taşıma aracı, diğer veri yönetim araçlarıyla aynı FastComments iş işleme sistemini kullanır.

Taşımanız başlamadan önce bir gecikme olabilir. Bu normaldir, çünkü sistem periyodik olarak işlenecek yeni işleri kontrol eder.

İş çalıştıkça, taşınacak bulunan yorum sayısını ve şu ana kadar taşınan yorum sayısını gösterecektir.

### API ile Alan Taşıma

Alanlar, `type` değerini `domain` olarak ayarlayarak [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post) ile de taşınabilir. İlerleme, [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id) üzerinden alınabilir ve taşıma 90 gün boyunca geri alınabilir.

---