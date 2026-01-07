### Geliştiriciler için - Karanlık modu zorla kapatma

Karanlık modu zorla kapatmak, widget yapılandırmasında `hasDarkBackground` değerini `false` olarak geçirerek yapılabilir. Bu, VanillaJS, Angular, React, Vue ve React Native kütüphaneleri için çalışır.

Her kütüphanenin [GitHub](https://github.com/fastComments/)'da karanlık modun nasıl kullanılacağına dair örnekler içeren bir `examples` klasörü vardır.

### Karanlık modu zorla açma

`hasDarkBackground` değerini `true` olarak ayarlayarak karanlık modun her zaman açık olmasını zorlayabiliriz.

Bunu Widget Özelleştirme arayüzü üzerinden de [buradan](https://fastcomments.com/auth/my-account/customize-widget) yapabiliriz.

`Base Theme` altında sadece `Force Dark Mode` seçeneğini seçin.

### VanillaJS Widget - Karanlık modu güncelleme

Karanlık modu güncellemenin en kolay yolu, sayfadaki tüm widget örneklerini geçip yapılandırmalarını güncellemektir:

[inline-code-attrs-start title = 'VanillaJS - Dark Mode Toggle'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    let isDarkMode = somehowDetermineIfDarkModeEnabled();
    for (const instanceWrapped of window.fcUIInstances) {
        if (instanceWrapped.targetElement) {
            const config = instanceWrapped.config;
            config.hasDarkBackground = isDarkMode;
            instanceWrapped.instance.update(config)
        }
    }
[inline-code-end]
