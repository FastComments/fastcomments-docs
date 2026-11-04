Son Tartışmalar widget'ı, sitenizdeki en son yorum etkinliğine sahip sayfaları görüntüler. Hâlâ yeni yorumların eklendiği konuları öne çıkarmak için kullanışlıdır; böylece ziyaretçiler sakin sayfalara düşmek yerine aktif sohbetlere geri dönebilir.

## Seçenekler

- **Başlık** (isteğe bağlı): Listenin üzerinde gösterilen başlık. Varsayılan olarak "Son Tartışmalar" şeklindedir.
- **Adet** (isteğe bağlı): Kaç tartışma gösterileceği. 1 ile 50 arası. Varsayılan 20.

## Nasıl Eklenir

### Bir Yazı veya Sayfa İçinde

Blok editöründe bir **Shortcode** bloğu ekleyin ve şunu yapıştırın:

[inline-code-attrs-start title = 'Son Tartışmalar kısa kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

`count` özniteliği 1 ile 50 arasında herhangi bir değeri kabul eder.

### Kenar Çubuğunda veya Alt Bilgide (Klasik Temalar)

WordPress yönetim panelinde **Görünüm > Bileşenler** bölümüne gidin. Blok ekleyiciden "FastComments" araması yapın ve **FastComments: Son Tartışmalar** seçeneğini seçin. Bunu bir kenar çubuğuna, başlığa veya alt bilgi alanına sürükleyin, ardından bileşen panelinden başlığı ve adedi yapılandırın.

### Blok Temasında (Tam Site Düzenleme)

**Görünüm > Düzenleyici** altındaki **Site Düzenleyici**'yi açın. Widget'ın görünmesi gereken şablon bölümüne gidin, bir **Legacy Widget** bloğu ekleyin ve açılır menüden **FastComments: Son Tartışmalar** seçeneğini seçin.

## Sorun Giderme

Widget yalnızca FastComments kurulumu tamamlandıktan ve bir tenant ID saklandıktan sonra görüntülenir. Widget alanı boşsa, WordPress yönetim panelinde **FastComments** altındaki kurulumu tamamlayın ve sayfayı yeniden yükleyin.

Tartışma sıralaması eski görünüyorsa, ilgili sayfaların FastComments panosunda senkronizasyonunun tamamlandığını kontrol edin. Widget canlı verileri okur, bu nedenle yeni içe aktarılan yorumların görünmesi biraz zaman alabilir.