İnsanların bir web sitesine içerik eklemesine izin verildiğinde
ve sonra bu içeriği birçok farklı cihaz türünde görüntülediğinizde güvenlikle ilgili birden fazla yön vardır.

### Biçimlendirme Suistimalinin Önlenmesi

İnsanlar, metin biçimlendirmesini kötüye kullanarak kasıtlı olarak görsel olarak dikkat dağıtan
ve tartışmalardan değeri azaltan içerikler yazabilir.

FastComments, biçimlendirmeyle ilgili suistimalleri önlemek için bir dizi önlem alır:

- Ardışık tekrar eden fazla satır sonları birleştirilir.
- Başlıkları render etmiyoruz (normal metne dönüşürler).
- CSS'e veya özel renklere izin vermiyoruz.

### İstismarların Önlenmesi

HTML render eden sistemlerde istismar oluşturulabilir. FastComments bunu önlemek için birkaç şey yapar:

- Açıkça tanımlanmış bir HTML etiketleri kümesine izin veriyoruz.
- Açıkça tanımlanmış bir HTML etiket öznitelikleri kümesine izin veriyoruz.
- Tüm girdileri temizler ve arındırırız.
  - Bu, [DOMPurify](https://www.npmjs.com/package/dompurify) ve [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) kütüphaneleri aracılığıyla yapılır.
  - Bu kütüphaneleri iyi test edilmiş oldukları için seçtik (sırasıyla haftada 4 ve 1 milyondan fazla indirme alıyorlar).

Bu, kullanıcıların `<script>` veya `<style>` etiketleri yazması ya da görüntülere veya diğer içeriğe `onload=alert()` türü betikler eklemeye çalışması gibi şeyler yapamayacağı anlamına gelir.

İzin verdiğimiz HTML etiketleri şunlardır:

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

`<iframe>` etiketi varsayılan olarak izin verilmez. Eğer Allow Media Embeds'i açarsanız, iframe'lere de izin verilir, fakat yalnızca kaynakları yerleşik güvenilir sağlayıcılar listesinden biri (ör. YouTube, Vimeo, SoundCloud ve Spotify) veya sizin açıkça eklediğiniz bir alan adı olduğunda. Diğer herhangi bir kaynaktan gelen iframe'ler kaldırılır.