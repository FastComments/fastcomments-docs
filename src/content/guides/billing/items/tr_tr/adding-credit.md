Bakiye yüklemeleri hesabınıza kaydedilmiş ödeme yönteminden tahsil edilir. Henüz bir kart kaydetmediyseniz, Krediler sayfası `Add a payment method first` mesajını ve bir `Go to billing` düğmesini gösterir. Önce [Billing Info](https://fastcomments.com/auth/my-account/billing-info) sayfasından bir kart kaydedin, sonra geri dönün.

Kredi eklemek için:

1. [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) sayfasını açın ve `Manage Credits` düğmesine tıklayın.
2. `Add credit` bölümünde hazır tutarlardan birini seçin veya `Other amount` seçeneğini seçip kendi tutarınızı girin.
3. `Add` düğmesine tıklayın. Düğme seçtiğiniz tutarı gösterir, örneğin `Add $25.00`.
4. Bankanız ödemeyi onaylamanızı istiyorsa, açılan pencerede doğrulamayı tamamlayın.

Ödeme tamamlandığında sayfa yeni bakiyenizle birlikte `Credit added` mesajını gösterir ve yükleme `Past top-ups` bölümünde `Credited` olarak görünür. Tekrar yükleme yapmak için `Add more` düğmesine tıklayın.

### Tutarlar ve Limitler

Hazır tutarlar faturalama para biriminizde `10`, `25`, `50` ve `100` şeklindedir. Özel bir tutar her yükleme için `5` ile `10,000` arasında olabilir ve para biriminize ait kesin limitler formun altında gösterilir. Japon yenisi gibi ondalık basamağı olmayan para birimlerinde, hazır tutarlar ve limitler yaklaşık aynı parayı temsil etmesi için yüz kat daha büyüktür.

Yüklemeler her zaman hesabınızın faturalama para biriminde, yani faturalarınızla aynı para biriminde yapılır. Sayfa hangi para biriminin tahsil edileceğini belirtir.

### Banka Onayı (3D Secure)

Bazı bankalar çevrimiçi ödemeleri onaylamanızı ister; bu genellikle bankacılık uygulamanız veya tek kullanımlık bir kod aracılığıyla yapılır. Sizin bankanız da bunu gerektiriyorsa, `Add` düğmesine tıkladıktan sonra bir onay penceresi açılır. Kartınız yalnızca banka onayladıktan sonra tahsil edilir ve bakiyeniz yalnızca o zaman güncellenir.

Onayı tamamlamadan önce sayfayı kapatırsanız, yükleme `Pending` olarak kaydedilir. Krediler sayfasını bir sonraki açışınızda ödemenin durumunu Stripe ile kontrol eder ve buna göre `Credited` veya `Failed` olarak işaretleriz. Banka ödemeyi onaylamazsa, sayfa `Your bank did not approve the payment. Try again, or use a different card.` mesajını gösterir ve hiçbir tahsilat yapılmaz.

### Reddedilen Kartlar

Kartınız reddedilirse, sayfa `Your card was declined. Check your card on the billing page, or try another card.` mesajını gösterir. Hiçbir tahsilat yapılmaz, bakiyeniz değişmez ve deneme `Past top-ups` bölümünde `Failed` olarak listelenir. Kartı [Billing Info](https://fastcomments.com/auth/my-account/billing-info) sayfasında güncelleyin ve tekrar deneyin.

Kartınızı korumak için bir hesap saat başına sınırlı sayıda yükleme denemesi yapabilir. `Too many top-up attempts. Wait a while and try again.` mesajını görürseniz, bekleyin ve daha sonra tekrar deneyin.

Ödemeler Stripe tarafından işlenir. FastComments kart bilgilerinizi asla saklamaz.