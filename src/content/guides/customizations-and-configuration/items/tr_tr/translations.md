[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

FastComments ile yorum bileşenindeki tüm metinler özelleştirilebilir.

Gönderme düğmesi gibi tek bir metni veya tüm yorum bileşenindeki tüm metinleri değiştirebilirsiniz.

Varsayılan olarak, yorum bileşenindeki metin kullanıcıların yerel ayarlarına göre çevrilir. Ancak, kullanıcı tabanımızın aynı yerel/dili kullandığından eminseniz
metni geçersiz kılabiliriz, örneğin:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Tüm özelleştirilebilir çeviriler <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">burada</a> "gelişmiş seçenekler" sekmesi altında bulunur.

Ancak, widget özelleştirme kullanıcı arayüzü üzerinden daha kolay bir yol var. Orada, yorum bileşeninde EN_US yerel ayarında gösterilen metni bulup,
bir değişiklik belirtebiliriz.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Tüm çeviri geçersiz kılmaları şu anda tüm yerel ayarları etkiler.