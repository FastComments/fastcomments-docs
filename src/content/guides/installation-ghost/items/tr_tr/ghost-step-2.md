Zip dosyasını indirdikten sonra bir klasöre çıkartın. Varsayılan `casper.zip` dosyasını indirdim ve Windows'ta `Downloads\casper` klasörüne çıkardım.

Ardından, LTS veya daha yeni bir NodeJS sürümünün yüklü olduğundan emin olmanız gerekiyor. Buradan indirebilirsiniz: https://nodejs.org/en/download/

NodeJS yüklendikten sonra bir kod düzenleyici yüklemeniz gerekecek.

30 günlük deneme sürümüyle (kredi kartı gerekmez) buradan edinebileceğiniz Webstorm'u öneriyoruz (ve kullanıyoruz): https://www.jetbrains.com/webstorm/

Muhtemelen bir sonraki en iyi ücretsiz seçenek Visual Studio Code olacaktır: https://code.visualstudio.com/download

Düzenleyicinizi kurup tema klasörünü düzenleyicide açtıktan sonra, IDE'deki terminali açın ve şunu çalıştırın:

[inline-code-attrs-start title = 'Temayı Yükle'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

Başarılı çıktı şöyle görünecektir (uyarıları yok sayabilirsiniz):

<div class="screenshot white-bg">
    <div class="title">Başarılı npm install çıktısı</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Başarılı npm install çıktısı" />
</div>

Bu, daha sonra çalıştıracağımız komutlar için temanın bağımlılıklarını kuracaktır. Ayrıca, dışa aktarma işleminin çalışması temanın bağımlılıklarının yüklü olmasına bağlıdır, aksi takdirde yeniden içe aktarma düzgün çalışmayacaktır.