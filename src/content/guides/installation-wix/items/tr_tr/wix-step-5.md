Şimdi, yorum dizisinin geçerli sayfaya göre değişmesi için ayarları yapalım; böylece kullanıcılar o anda görüntülenen içeriği tartışabilir.

Aşağıdaki adımları uygulamazsanız, tüm siteniz için yalnızca tek bir global yorum dizisine sahip olursunuz - ki bu pek kullanışlı değildir.

#### Dev Mode

Bu işlevselliği eklemek için Wix'in `Dev Mode` olarak adlandırdığı moda girmemiz gerekiyor.

Ekranın üst kısmındaki `Dev Mode` seçeneğine tıklayın.

<div class="screenshot white-bg">
    <div class="title">Dev Mode'u Etkinleştir</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Dev Mode'u Etkinleştir" />
</div>

#### Element ID'sini Ayarlayın

Bunu başarmak için özel kod ekleyeceğiz, ancak önce yeni gömme (embed) elementine, ona atıfta bulunmak için bir ID vermemiz gerekiyor.

Buna `fastcomments` diyelim.

Eklediğimiz yeni gömme elementine tıklayın; dev modunda sağ altta `html1` gibi bir değere sahip bir ID alanı görmelisiniz:

<div class="screenshot white-bg">
    <div class="title">ID Alanı</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="ID Alanı" />
</div>

Bunu `fastcomments` olarak değiştirin ve enter tuşuna basın:

<div class="screenshot white-bg">
    <div class="title">ID'yi Ayarla</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="ID'yi Ayarla" />
</div>

Artık hangi sayfayı görüntülediğimizi yorum alanına söyleyen özel kodumuzu ekleyebiliriz.

Ekranın alt kısmında şuna benzer bir kod düzenleyicisi görmelisiniz:

<div class="screenshot white-bg">
    <div class="title">Düzenleyiciyi Aç</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="ID'yi Ayarla" />
</div>

Aşağıdaki kodu kopyalayın ve oraya yapıştırın:

[inline-code-attrs-start title = 'Wix Yorum Gezinme Parçacığı'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Gezinme Kodunu Ekle</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Gezinme Kodunu Ekle" />
</div>