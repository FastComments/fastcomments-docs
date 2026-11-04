Bir ajan üç durumdan birine sahip olabilir:

### Devre Dışı (Disabled)

Ajan kapatılmıştır. Hiçbir tetikleyici işlenmez ve ajan dağıtım yolunda görünmez. Çalışma geçmişi, analitikler ve bellek korunur - daha sonra yeniden etkinleştirirseniz, geçmiş veriler hâlâ oradadır.

`Disabled` durumunu şu durumlarda kullanın:
- Bir ajanı kaybetmeden devreden çıkarmak istediğinizde.
- Bir ajan yanlış davranıyordur ve araştırma yaparken hemen durdurmanız gerektiğinde.
- Ajanları mevsimsel olarak devreye alıp çıkardığınızda (örn. yalnızca tatillerde çalışan bir karşılama ajanı).

### Dry Run - yeni ajanlar için varsayılan

Ajan uçtan uca çalışır - tetikleyicileri işler, LLM'i çağırır, araç çağrılarını seçer, gerekçeleri ve güven düzeylerini hesaplar - ancak **gerçek bir eylem yapılmaz**. Her çalışma, [Run History](#run-history) bölümünde **Dry Run** rozetiyle kaydedilir.

`Dry Run` durumunu şu durumlarda kullanın:
- Yeni bir ajan daha kutusundan yeni çıkmıştır. Her başlangıç şablonu dry-run modunda gelir.
- Prompt'u düzenlediyseniz veya tetikleyici setini değiştirdiyseniz ve taahhüt etmeden önce değişikliğin nasıl sonuçlanacağını görmek istiyorsanız.
- Bir [test çalışması / yeniden oynatma](#test-runs-replays) yürüttüğünüzde (yeniden oynatmalar, ajan durumundan bağımsız olarak dry-run'ı zorunlu kılar).

Platform, dry-run çalışmaları için token ücretlendirir - LLM çağrısı yine de gerçekleşir, yalnızca yan etkiler atlanır. Bütçe limitleri dry-run için de geçerlidir. Bkz. [Budgets Overview](#budgets-overview).

### Etkin (Enabled)

Ajan gerçek eylemler gerçekleştirir. Araç çağrıları yürütülür - veya eylem kısıtlanmışsa [onay](#approval-workflow) için kuyruğa alınır.

Dry-run çıktısı doğru göründükten sonra `Enabled` durumunu kullanın.

### Durum değiştirme

Düzenleme formunda herhangi iki durum arasında geçiş yapabilirsiniz. Dry Run'dan Enabled'a geçiş, dry-run eylemlerini geriye dönük olarak yeniden yürütmez - bunlar dry-run geçmişi olarak kalır. O andan itibaren gelen yeni tetikleyiciler canlı çalışır.

Çalışma sırasında Enabled'dan Disabled'a geçmek, devam eden bir çalışmayı **durdurmaz**. Şu anda yürütülmekte olan tetikleyici tamamlanır (zaten başlatıldığı şeylerle birlikte); sonraki tetikleyici, ajan artık Disabled olduğu için atılır.

### Faturalandırma sorunları sırasında durum

Kiracınızın (tenant) faturalandırması geçersiz hale gelirse, kaydedilmiş durumdan bağımsız olarak tüm ajanlar fiilen duraklatılır - faturalandırma geri yüklenene kadar tetikleyiciler `BILLING_INVALID` ile atılır. Kaydedilen durum alanı değiştirilmez; yalnızca dağıtıcı çalıştırmayı reddeder. Bkz. [Plans and Eligibility](#plans-and-eligibility).