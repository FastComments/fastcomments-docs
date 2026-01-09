Şimdi özel FastComments kodunuzu oluşturacağız. FastComments'in GoHighLevel sitenizde nasıl çalışmasını istediğinizi yapılandırmak için aşağıdaki sihirbazı kullanın:

[snippet id="gohighlevel-wizard"]

### Farklı Yorum Kutusu Türleri

Kullanılan ürünü değiştirmek için `TYPE = 'commenting'` satırını yapılandırabilirsiniz (örneğin, canlı akış sohbeti için `live` veya işbirlikçi sohbet için `collab` olarak değiştirebilirsiniz).

### Yorum Kutusunu İstediğiniz Yere Yerleştirme

Varsayalım ki yorum kutularını sayfanın belirli bölümlerine, varsayılan konumlar yerine yerleştirmek istiyorsunuz.
Bu satırı değiştirin:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

Şununla:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Ardından GHL düzenleyicisinde "code" butonuna tıklayın ve yorumların gitmesini istediğiniz yere ekleyin:

[inline-code-attrs-start title = 'GoHighLevel FastComments Div (div öğesi)'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Sayfa Başına Farklı Yorum Kutusu Türü

Varsayalım ki kullanıcıların metin parçalarını vurgulayıp tartışmasını veya bunun yerine akış sohbeti kullanıcı arayüzünü kullanmasını istiyorsunuz.

Önce "Yorum Kutusunu İstediğiniz Yere Yerleştirme" bölümündeki adımları izleyin.

O küçük kod parçasında `type="commenting"` olduğunu unutmayın.

Örneğin collab sohbetini etkinleştirmek isterseniz type'ı `type="collab"` olarak değiştirin.

### Sadece Belirli Sayfalarda Göster

Eğer `TARGET_ELEMENT_ID`'yi ayarlamazsanız, bunun yerine hangi URL yollarında yorumların gösterileceğini belirlemek için `VALID_PATTERNS` değişkenini yapılandırabilirsiniz. Varsayılan olarak, URL'de `/post` içeren sayfalarda gösterir.

### Collab Sohbetini Yapılandırma

Collab sohbetine, işbirlikçi işlevselliği yalnızca belirli bir alan içindeki HTML etrafına eklemesini söyleyebilirsiniz; örneğin, diyelim ki siz
yukarıdaki footer kodunu eklediniz ve ardından collab sohbetini etkinleştirmek için post/sayfa içeriğine bu div'i eklediniz:

[inline-code-attrs-start title = 'Belirli İçerikle Collab Sohbeti'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Daha sonra `<div>` içindeki paragraf öğesinde collab sohbeti etkinleştirilecek, ve sayfadaki diğer hiçbir şey etkilenmeyecek. Eğer
`<div>` içine hiç içerik koymazsanız o zaman collab sohbeti tüm gönderi gövdesinde etkinleştirilir.

---