Bir ödeme, kartın süresi dolmuş, değiştirilmiş, bakiyesi olmaması veya bankanın işlemi engellemiş olması nedeniyle başarısız olabilir. FastComments, bir şey çalışmayı bırakmadan önce sorunu düzeltmeniz için size zaman tanır ve sorunu çözebilecek kişilere haber verir.

### Ne Olur

1. **Tahsilat başarısız olur.** Fatura, [Billing History](https://fastcomments.com/auth/my-account/billing-history) sayfasında `Pay Now` düğmesiyle birlikte `Unpaid` durumunda kalır. Sabit bir planda Stripe tahsilatı yeniden deneyebilir. Flex'te FastComments, ödemesi yapılmamış ve vade süresinden daha eski olan her faturayı her gece yeniden tahsil etmeyi dener.
2. **Bir uyarı gönderilir.** FastComments her gece tüm hesapları kontrol eder. Bir Flex faturası vade süresi geçmiş olmasına rağmen hâlâ ödenmemişse ve yeniden deneme başarısız olursa ya da tahsil edilemediği için sabit planlı bir abonelik sona ermişse, `Billing Admin` iznine sahip her kullanıcı, tarihi de içeren `Your FastComments Tenant Will Be Deactivated Soon!` konulu e-postayı alır ve hesap, bir vade süresi sonra devre dışı bırakılmak üzere planlanır. Bu süreçte widget çalışmaya devam eder.
3. **Devre dışı bırakma.** Planlanan tarihe kadar hiçbir şey değişmemişse hesap devre dışı bırakılır: yorumlar sitenizde gizlenir ve ziyaretçiler `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` mesajını görür. Faturalandırma yöneticilerine tekrar e-posta gönderilir. Verileriniz saklanır.

Vade süresi çoğu hesap için bir hafta, bir yıldan eski hesaplar için iki haftadır. Flex'te bu süre iki kez uygulanır: biri uyarıdan önce, diğeri uyarı ile devre dışı bırakma arasındadır; böylece başarısız bir tahsilat ile yorumların gizlenmesi arasında en az iki hafta olur.

### Nasıl Düzeltilir

[Billing Info](https://fastcomments.com/auth/my-account/billing-info) sayfasında çalışan bir kart kaydedin. Bu tek başına hesabı hemen yeniden etkinleştirir ve planlanmış tüm devre dışı bırakmaları iptal eder; ardından ödenmemiş fatura yeni kartla, sabit planda hemen ve Flex'te gece kontrolü sırasında tahsil edilir. Ayrıca faturayı doğrudan Billing History sayfasındaki `Pay Now` ile ödeyebilirsiniz.

Dosyadaki kart geçerliyse ve başarısızlık geçiciyse herhangi bir şey yapmanız gerekmez: gece kontrolü ödenmiş faturayı veya aktif aboneliği fark eder, devre dışı bırakmayı iptal eder ve onaylamak için faturalandırma yöneticilerine e-posta gönderir. Zaten devre dışı bırakılmış bir hesap da aynı şekilde yeniden etkinleştirilir.

### Aynı Anda Birden Fazla Fatura

Kartınız birkaç ay boyunca geçersizdi ve sonra düzeltildiyse, ödenmemiş her ay ayrı ayrı tahsil edilir; bu nedenle aynı gün birden fazla tahsilat görebilirsiniz. Billing History'deki her faturanın faturalandırma dönemini kontrol edin: bunlar farklı aylara aittir, kopya değildir.

### Ödeme Yaptıktan Sonra Hâlâ Bir Faturalandırma Uyarısı mı Görüyorsunuz?

Uyarı belirli bir hesaba aittir. Ödeme yaptıysanız ancak bir site hâlâ `billing information is out of date` gösteriyorsa, o sitedeki widget büyük olasılıkla ödeme yaptığınız hesaptan farklı bir hesap kullanıyordur. Bu, bir site iki kez kurulduğunda ya da `fastcomments.com` üzerinde bir hesap ve ayrı olarak faturalandırılan `eu.fastcomments.com` üzerindeki AB dağıtımında başka bir hesap bulunduğunda olur. Sayfanızdaki koddaki `tenantId` değerini `Settings -> API/SSO` altındaki değerle karşılaştırın ya da [destek ekibiyle iletişime geçin](https://fastcomments.com/auth/my-account/help), sorunu çözelim.