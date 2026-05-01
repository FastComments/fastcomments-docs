The Recent Discussions widget, sitenizde en son yorum etkinliği olan sayfaları görüntüler. Hala yorum eklenmeye devam edilen başlıkları vurgulamak için kullanışlıdır; böylece ziyaretçiler sessiz sayfalara gelmek yerine aktif konuşmalara geri dönebilirler.

## Options

- **Title** (optional): Listenin üzerinde gösterilen başlık. Varsayılan "Son Tartışmalar".
- **Count** (optional): Kaç tartışma gösterileceği. Aralık 1 ile 50 arasındadır. Varsayılan 20.

## How to Add It

### Inside a Post or Page

Blok düzenleyicide, bir **Shortcode** bloğu ekleyin ve aşağıyı yapıştırın:

[inline-code-attrs-start title = 'Son Tartışmalar kısa kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

`count` özniteliği 1 ile 50 arasındaki herhangi bir değeri kabul eder.

### In a Sidebar or Footer (Classic Themes)

WordPress yönetiminde **Görünüm > Bileşenler** bölümüne gidin. Blok ekleyicisinden "FastComments" araması yapın ve **FastComments: Son Tartışmalar** öğesini seçin. Bunu bir kenar çubuğuna, üstbilgiye veya altbilgi alanına sürükleyin, ardından widget panelinden başlığı ve sayıyı yapılandırın.

### In a Block Theme (Full Site Editing)

**Görünüm > Düzenleyici** altında **Site Editor**'ü açın. Widget'ın görünmesi gereken şablon parçasına gidin, bir **Legacy Widget** bloğu ekleyin ve açılır menüden **FastComments: Son Tartışmalar** öğesini seçin.

## Troubleshooting

Widget, FastComments kurulumu tamamlandıktan ve bir tenant ID saklandıktan sonra görüntülenir. Eğer widget alanı boşsa, WordPress yönetimindeki **FastComments** altında kurulumu tamamlayın ve sayfayı yeniden yükleyin.

Tartışma sıralaması eski görünüyorsa, altta yatan sayfaların FastComments kontrol panelinde senkronizasyonunun tamamlandığını kontrol edin. Widget canlı verileri okur, bu yüzden yeni içe aktarılan yorumların görünmesi biraz zaman alabilir.