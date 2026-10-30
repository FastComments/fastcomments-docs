AI Ajanları **Flex** ve **Pro** planlarında mevcuttur. Creator planı bunları içermez.

### Plan düzeyinde limitler

Her plan seviyesi şunları belirler:

- **Varsayılan günlük ve aylık bütçe limitleri.** Bunları ajan başına düşürebilirsiniz; hesap başına limiti artırmak daha yüksek bir tavanı olan bir plan gerektirir. Bkz. [Budgets Overview](#budgets-overview).

Tam sayılar [fiyatlandırma sayfasında](https://fastcomments.com/traffic-pricing) ve hesabınızın faturalama sayfasında gösterilir. Ayrıca ajan düzenleme formunda satır içi olarak gösterilir, böylece limitinizi bulmak için formdan çıkmak zorunda kalmazsınız.

FastComments Pro, ayda $200 değerinde AI kullanımını içerir. Flex, tüm modeller için (şu anda GLM 5.2 veya gpt-oss-120B) milyon token başına $20 oranında faturalandırılır.

### Faturalamanın geçerli olması gerekir

AI Ajanları yalnızca kiracının **dosyada geçerli bir faturalaması** olduğunda çalışır. Ödeme yöntemi geçersiz hale gelirse, tüm ajanlar duraklatılır ve AI Ajanları sayfası, **Billing Admin** rolüne sahip olan kişiyi faturalamayı güncellemeye yönlendiren bir banner gösterir. Faturalama geri yüklendiğinde ajanlar kendi kendine devam eder – kesinti sırasında tetiklenen tetikleyicilerin yeniden oynatılması veya geri doldurulması yapılmaz.

Bu katı bir ön koşuldur: token harcaması hesabınıza fatura edilir, bu nedenle platform, çalışan bir ödeme yöntemi olmadan herhangi bir LLM çağrısı göndermez.

### Ajanları kim yönetebilir

Ajan yönetim sayfaları **Customization Admin** kontrol paneli rolünün arkasında kilitlenmiştir. **Comment Moderator Admins** onayları gözden geçirebilir ve karar verebilir (bkz. [Approval Workflow](#approval-workflow)), ancak ajan oluşturamaz veya düzenleyemez. **Billing Admins**, ajan erişimi olup olmamasına bakılmaksızın [budget alert emails](#budget-alerts) alır.