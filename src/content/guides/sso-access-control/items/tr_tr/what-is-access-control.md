FastComments SSO Erişim Kontrolü ile, bazen RBAC olarak da adlandırılır, kullanıcılara yalnızca belirli sayfalara veya yorum dizilerine erişme kısıtlaması getirilebilir. Ayrıca kullanıcılar yalnızca aynı gruptaki diğer kullanıcıları `@mention` edebilir.

## Ayrıntılar

`Users` ve isteğe bağlı olarak `Pages`'i gruplara yerleştirebiliriz.

`Users` gruplara yerleştirildiğinde ve Widget Ayarlarında `Limit Comments by SSO User Groups` etkinleştirildiğinde, kullanıcılar yalnızca kendi gruplarındaki kullanıcıların yorumlarını görecek ve yalnızca aynı gruptaki kullanıcıları `@mention` edebileceklerdir.

Ek olarak, `Pages` gruplara yerleştirilebilir ve bu durumda kullanıcılar yalnızca erişime sahip oldukları sayfaların yorumlarına erişebilirler.

Buna "User-Level" grupları ile "Page-Level" grupları olarak atıfta bulunuyoruz.

Peki hangisi sizin için uygun?

#### Kullanıcı Düzeyi Gruplarını kullanın eğer...

- Aynı `urlId` değerini (sayfa URL'si veya makale ID'si) kullanmak istiyorsunuz, ancak yorumları gruba göre kısıtlamak istiyorsunuz.
- Örneğin, "New User" ve "Veteran User" gruplarınız olsun ve aynı sayfalarda birbirlerinin yorumlarını hiçbir zaman görmemeliler.

#### Sayfa Düzeyi Gruplarını kullanın eğer...

- Gruplarınızın belirli sayfaları var.
- Örneğin, "Public Pages" grubundaki kullanıcılar "Top Secret" makalelerini asla görmemelidir.