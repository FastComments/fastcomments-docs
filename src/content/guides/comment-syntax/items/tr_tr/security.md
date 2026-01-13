İnsanların bir web sitesine içerik eklemesine ve ardından bu içeriği birçok farklı cihaz türünde görüntülemesine izin verildiğinde güvenliğin birden fazla yönü vardır.

### Biçimlendirme kötüye kullanımını önleme

İnsanlar, metin biçimlendirmesini kötüye kullanarak kasıtlı olarak görsel açıdan dikkat dağıtıcı ve tartışmaların değerini azaltan içerikler yazabilir.

FastComments, biçimlendirmeyle ilgili kötüye kullanımı önlemek için bir dizi şey yapar:

- Büyük tekrarlanan ardışık satır sonları daraltılır.
- Başlıkları render etmiyoruz (normal metin haline gelirler).
- CSS veya özel renklere izin vermiyoruz.

### Açıkları önleme

HTML render eden sistemlerde açıklar oluşturulabilir. FastComments bunu önlemek için çeşitli şeyler yapar:

- Yalnızca açıkça tanımlanmış bir HTML etiketi kümesine izin veriyoruz.
- Yalnızca açıkça tanımlanmış bir HTML etiketi özniteliği kümesine izin veriyoruz.
- Tüm girdileri arındırıyor ve sterilize ediyoruz.
  - Bu, [DOMPurify](https://www.npmjs.com/package/dompurify) ve [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) kütüphaneleri aracılığıyla yapılır.
  - Bu kütüphaneleri iyi test edilmiş oldukları için seçtik (sırasıyla haftada 4 ve 1 milyonun üzerinde indirme ile).

Bu, kullanıcıların `<script>` veya `<style>` etiketleri yazma veya resimlere veya diğer içeriklere `onload=alert()` türü komut dosyaları eklemeye çalışma gibi şeyler yapamayacağı anlamına gelir.

İzin verdiğimiz HTML etiketleri aşağıdaki gibidir:

- `<b>`
- `<em>`
- `<u>`
- `<i>`
- `<strike>`
- `<pre>`
- `<span>`
- `<code>`
- `<img>`
- `<a>`
- `<strong>`
- `<ul>`
- `<ol>`
- `<li>`
- `<br>`
