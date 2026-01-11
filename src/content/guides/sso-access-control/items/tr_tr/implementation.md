#### Diğer Gruplardaki Kullanıcıları `@mention` Etme

Eğer iki kullanıcı iki farklı grup kümesine aitse ve kesişimleri yoksa, birbirlerini `@mention` yapamazlar.

Bir kullanıcı manuel olarak bir `@mention` yazıp yorumunu gönderirse, bu düz metin olarak kalır. Diğer kullanıcı etiketlenmez.

#### Grupları Yönetme

`Groups`, sırasıyla `Pages` ve `SSOUsers` API kaynakları kullanılarak tanımlanır.

`Pages` API'si, sayfaya erişmesine izin verilen grup kümesini tanımlamak için çağrılabilir. Varsayılan olarak tüm grupların ve herhangi bir gruba ait
olmayan kullanıcıların erişimi vardır.

Benzer şekilde, `SSOUsers` API'si her kullanıcıyla ilişkili grupları tanımlamak için çağrılabilir.

Her iki kaynak için de grupların ne zaman ayarlanabileceği veya güncellenebileceği konusunda herhangi bir kısıtlama yoktur.

Eğer amaç yalnızca kullanıcıların birbirlerini `@mention` yapmasını sınırlamaksa, `Pages` dikkate alınmak zorunda değildir.

##### Not!

SSO kullanıcı gruplarını tanımlamak ve güncellemek API kullanmayı gerektirmez; bunun yerine yorum bileşenine geçirilen SSO yükünde
grup kimliklerini tanımlayarak otomatik olarak güncellenebilir. Ancak, grup listeleri büyükse bu önerilmez çünkü kullanıcı
her sayfa yüklemesi için bu yükü göndermek zorunda kalır.

---