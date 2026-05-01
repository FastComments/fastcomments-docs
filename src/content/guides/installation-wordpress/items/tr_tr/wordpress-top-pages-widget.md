Top Pages bileşeni sitenizdeki en çok yorum alan sayfaları gösterir. Yeni ziyaretçilere en çok etkileşim alan içeriğinizi öne çıkarmak ve sitede geçirilen süreyi artırmak için kullanışlıdır.

## Seçenekler

- **Başlık** (isteğe bağlı): Listenin üzerinde gösterilen başlık. Varsayılan: "En Çok Yorumlanan Sayfalar".

Top Pages bileşeni kullanılabilir verilere göre kendi düzenini seçer ve bir count özniteliğini kabul etmez.

## Nasıl Eklenir

### Bir Gönderi veya Sayfa İçine

Blok düzenleyicisinde, bir **Kısa Kod** bloğu ekleyin ve yapıştırın:

[inline-code-attrs-start title = 'En Çok Yorumlanan Sayfalar kısa kodu'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### Bir Kenar Çubuğunda veya Alt Bilgide (Klasik Temalar)

WordPress yönetici panelinizde **Görünüm > Bileşenler** sayfasına gidin. Blok ekleyicisinden "FastComments" araması yapın ve **FastComments: Top Pages** öğesini seçin. Bunu bir kenar çubuğuna, üst bilgiye veya alt bilgi alanına sürükleyin, ardından başlığı bileşen panelinden ayarlayın.

### Bir Blok Temasında (Tam Site Düzenleme)

**Görünüm > Düzenleyici** altında **Site Düzenleyicisi**ni açın. Bileşenin görünmesi gereken şablon bölümüne gidin, bir **Legacy Widget** bloğu ekleyin ve açılır menüden **FastComments: Top Pages** öğesini seçin.

## Sorun Giderme

Bileşen, yalnızca FastComments kurulumu tamamlandıktan ve bir tenant ID saklandıktan sonra render edilir. Eğer bileşen alanı boşsa, WordPress yönetici panelinde **FastComments** altında kurulumu tamamlayın ve sayfayı yeniden yükleyin.