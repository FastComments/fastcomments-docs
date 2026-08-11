[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

FastComments ile yorum widget'ındaki tüm metin özelleştirilebilir.

Gönder düğmesi gibi tek bir metni ya da tüm yorum widget'ındaki tüm metni geçersiz kılabilirsiniz.

Varsayılan olarak, yorum widget'ındaki metin kullanıcının yerel ayarına göre çevrilir. Ancak, kullanıcı tabanımızın aynı yerel/ dili kullandığından eminsek, metni geçersiz kılabiliriz, örneğin:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Tüm özelleştirilebilir çeviriler <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">burada</a> 'gelişmiş seçenekler' sekmesi altında bulunabilir.

Ancak, widget özelleştirme UI'sı üzerinden daha kolay bir yol vardır. Orada, EN_US yerel ayarında yorum widget'ında gösterilen metni bulup bir değiştirme belirtebiliriz.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Açılır menüden seçilen bir widget dizesi ve bir değiştirme metin alanı içeren özel metin paneli'; title='Özel Metin' app-screenshot-end]

Tüm çeviri geçersiz kılmaları şu anda tüm yerel ayarları etkiler.