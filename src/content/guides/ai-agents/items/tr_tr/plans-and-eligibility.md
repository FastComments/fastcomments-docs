AI Ajanları **Flex** ve **Pro** planlarında mevcuttur. Creator planı bunları içermez.

### Plan düzeyi sınırları

Her plan seviyesi şunları belirler:

- **Varsayılan günlük ve aylık bütçe üst limitleri.** Bunları her ajan için düşürebilirsiniz; hesap başına üst limiti yükseltmek daha yüksek sınır sunan bir plan gerektirir. Bakınız [Bütçeler Genel Bakışı](#budgets-overview).

Kesin rakamlar [fiyatlandırma sayfasında](https://fastcomments.com/traffic-pricing) ve hesabınızın faturalama sayfasında gösterilir. Ayrıca ajanın düzenleme formunda satır içi olarak gösterilir, böylece üst limitinizi bulmak için formdan ayrılmanız gerekmez.

FastComments Pro, aylık 200$/ay değerinde AI kullanımı içerir. Flex için tüm modellerde (şu anda GLM 5.1 veya gpt-oss-120B-turbo) milyon başına 20$ üzerinden faturalandırılır.

### Faturalandırma geçerli olmalıdır

AI Ajanları yalnızca kiracıda **kayıtlarda geçerli fatura bilgisi** bulunduğunda çalışır. Ödeme yöntemi geçersiz hale gelirse, tüm ajanlar duraklatılır ve AI Agents sayfası, **Billing Admin** rolüne sahip kişiyi faturalamayı güncellemesi için yönlendiren bir banner gösterir. Faturalama geri yüklendiğinde ajanlar kendi kendine devam eder - kesinti sırasında tetiklenen olayların tekrar oynatılması veya geriye dönük doldurulması yapılmaz.

Bu katı bir ön koşuldur: token harcamaları hesabınıza faturalandırılır, bu nedenle platform, çalışan bir ödeme yöntemi olmadan herhangi bir LLM çağrısı göndermeyecektir.

### Ajanları kim yönetebilir

Ajan yönetim sayfalarına erişim **Customization Admin** dashboard rolü ile sınırlandırılmıştır. **Comment Moderator Admins** onayları inceleyip karar verebilir (bkz. [Onay İş Akışı](#approval-workflow)) ancak ajan oluşturamaz veya düzenleyemez. **Billing Admins** ajan erişimi olsun ya da olmasın [bütçe uyarı e-postaları](#budget-alerts) alır.