Son Yorumlar bileşeni, sitenizin tamamında yayınlanan en son yorumları gösterir. Kenar çubuklarında, altbilgilerde veya taze etkinliği öne çıkarıp daha fazla okumayı teşvik etmek istediğiniz herhangi bir yerde kullanışlıdır.

## Seçenekler

- **Title** (isteğe bağlı): Liste üzerinde gösterilen başlık. Varsayılan "Son Yorumlar".
- **Count** (isteğe bağlı): Kaç yorum gösterileceği. Aralık 1 ila 50. Varsayılan 5.

## Nasıl Eklenir

### Bir Yazı veya Sayfa İçinde

Blok editöründe, bir **Shortcode** bloğu ekleyin ve yapıştırın:

[inline-code-attrs-start title = 'Son Yorumlar kısa kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

`count` özniteliği 1 ile 50 arasındaki herhangi bir değeri kabul eder.

### Bir Kenar Çubuğunda veya Alt Bilgide (Klasik Temalar)

WordPress yönetiminizde **Görünüm > Bileşenler** bölümüne gidin. Blok ekleyicisinden "FastComments" arayın ve **FastComments: Son Yorumlar** öğesini seçin. Bir kenar çubuğuna, üst bilgiye veya alt bilgi alanına sürükleyin, ardından bileşen panelinden başlığı ve sayıyı yapılandırın.

### Bir Blok Temada (Tam Site Düzenleme)

**Görünüm > Düzenleyici** altındaki **Site Düzenleyicisi**ni açın. Bileşenin görünmesi gereken şablon parçasına gidin, bir **Legacy Widget** bloğu ekleyin ve açılır menüden **FastComments: Son Yorumlar** öğesini seçin.

## Sorun Giderme

Bileşen ancak FastComments kurulumu tamamlandıktan ve bir tenant ID saklandıktan sonra görüntülenir. Eğer bileşen alanı boşsa, WordPress yönetimindeki **FastComments** altında kurulumu tamamlayın ve sayfayı yeniden yükleyin.