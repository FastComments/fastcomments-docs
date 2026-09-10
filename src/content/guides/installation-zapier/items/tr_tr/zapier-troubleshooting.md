## Sorun Giderme

**"Bağlanırken \"İzin yok\" hatası alıyorsunuz".** Oturum açmış kullanıcı, hesapta API yöneticisi değil.  
Hesap sahibinden Kullanıcılar sayfasında API izni vermesini isteyin ya da sahibi olarak bağlanın.

**Bağlantı yanlış siteyle etiketlenmiş.** Onay sayfası, o anda oturum açtığınız hesabı bağlar.  
Zapier'de bağlantıyı kesin, FastComments kontrol panelinde hesabı değiştirin ve tekrar bağlanın.

**Olaylar gelmeyi durdurdu.** Kontrol panelindeki Webhooks sayfasını kontrol edin. Altı gün boyunca uç noktası başarısız olan bir abonelik otomatik olarak devre dışı bırakılır ve nedenini gösterir. Orada yeniden etkinleştirin ya da Zap'i kapatıp tekrar açın. Abonelik tamamen eksikse, birisi silmiştir; Zap'i kapatıp açmak onu yeniden oluşturur.

**Zapier, hesabın yeniden bağlanması gerektiğini söylüyor.** Bağlantı, Bağlı Uygulamalar sayfasından iptal edildi, onaylayan kullanıcı API iznini kaybetti ya da hesap silindi. Zapier'den yeniden bağlanın.

**Bir eylem "yazma erişimi yok" hatasıyla başarısız oluyor.** Bağlantı yalnızca okuma izniyle onaylandı. Yeniden bağlanın ve her iki izni de onaylayın.

**Hız sınırlamaları ve krediler.** Eylemler ve aramalar, planınızdaki API kredilerini harcar ve REST API ile aynı hız sınırlamalarına tabidir. Tetikleyiciler hiç harcamaz. Bir sınıra takılan Zap, FastComments'un bildirdiği gecikmeden sonra Zapier tarafından yeniden denenir.

**Domain açılır menüsü boş.** Domain'ler, FastComments kontrol panelindeki Domains sayfasında yapılandırıldıktan sonra görünür. Her domain için olay almak üzere alanı boş bırakın.