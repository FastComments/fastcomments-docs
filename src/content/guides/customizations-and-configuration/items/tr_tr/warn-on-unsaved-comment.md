[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, bir kullanıcı yorum yazıp gönderim yapmadan sayfayı yenilediğinde, sekmeyi kapattığında veya başka bir sayfaya gittiğinde, taslak sessizce kaybolur.

**warnOnUnsavedComment** özelliğini true olarak ayarlamak, bir yorum kutusu ya da devam eden bir düzenleme hâlâ metin içerdiği sürece, tarayıcının sayfadan ayrılmadan önce kullanıcıdan onay istemesini sağlar. Yorum gönderildiğinde metin temizlenir, bu yüzden herhangi bir uyarı gösterilmez.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Kaydedilmemiş Yorum Uyarısı'; code-example-end]

Uyarı, tarayıcının kendi iletişim kutusunu kullanır. Modern tarayıcılar kendi metinlerini gösterir ve özel metni yok sayar, bu yüzden mesaj özelleştirilemez.

Bu seçenek, talep üzerine küçük bir uzantı yükler, bu nedenle etkinleştirmeyen siteler için widget'a hiçbir şey eklemez.