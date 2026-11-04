Verilerin taşınması gerektiği durumlarda, FastComments, yorumları sayfalar ve makaleler arasında taşımayı sağlayan self servis bir araç sunar.

İşte yorum kopyalama sayfası formunun görünümü:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='From URL ID alanı ile To URL ID ve URL alanlarını içeren yorum kopyalama formu'; title='Yorum Kopyalama Formu' app-screenshot-end]

### "From" Alanlarının Doldurulması

Yorumların nereden taşınacağına karar vermek için, yalnızca kaynak `URL ID` bilgisine ihtiyacımız var.

Yorum widget yapılandırmasında `urlId` için bir değer iletmiyorsanız, bu, sayfa URL'sinin "temiz" bir sürümü olacaktır.

Yorumlarınızın `URL ID` değerlerinin ne olduğunu, bunları dışa aktararak görebilirsiniz.

### "To" Alanlarının Doldurulması

Yorumların nereye taşınacağına karar vermek için, hedef `URL ID` ve `URL` bilgisine ihtiyacımız var.

`URL ID`, yorumun gideceği kova (bucket) olacaktır. `URL` alanı, yorumlara e-postalardan ve moderasyon araçlarından doğrudan gidebilmeniz için kullanılır.

#### WordPress

WordPress kullanıyorsanız, örneğin taşıma aracındaki To/From `URL ID` alanlarına bir URL yerine Makale ID'lerini girmeniz gerekir.