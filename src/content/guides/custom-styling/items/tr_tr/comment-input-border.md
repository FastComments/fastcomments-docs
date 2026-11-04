Yorum kutusunun etrafındaki kenarlık, metin alanının kendi kenarlığından ve widget'ın etrafına çizdiği birkaç ince çizgiden oluşur. Rengini veya köşe yuvarlaklığını değiştirmek için `textarea` öğesini doğrudan stillendirmek yerine bu CSS değişkenlerini ayarlayın. Değişkenler kenarlığın her bölümünü aynı anda yeniden stillendirir, böylece köşeler ve renkler her zaman uyumlu olur.

| Değişken | Neyi değiştirir | Varsayılan |
|---|---|---|
| `--fc-input-border-color` | Kenarlık rengi | `#bfbfbf` |
| `--fc-input-border-color-focus` | Kullanıcı yazarken kenarlık rengi | `#555` |
| `--fc-input-border-radius` | Yuvarlatılmış köşelerin yuvarlaklığı | `11px` |
| `--fc-input-border-start-start-radius` | Kare sol üst köşenin yuvarlaklığı (sağdan sola dillerde sağ üst) | `0` |

CSS'i [Widget Özelleştirme sayfasındaki](https://fastcomments.com/auth/my-account/customize-widget) **Custom CSS** kutusuna ekleyin veya `customCSS` seçeneğiyle iletin. Yalnızca değiştirmek istediğiniz değişkenleri ayarlamanız yeterlidir.

Ayrıca Custom CSS kutusunun hemen altındaki **Comment box border** yardımcısını da kullanabilirsiniz; bu yardımcı bu CSS'i sizin için yazar.

## Kenarlık Rengini Değiştirme

[inline-code-attrs-start title = 'Kenarlık Rengi'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Yazarken Kenarlık Rengini Değiştirme

[inline-code-attrs-start title = 'Yazarken Kenarlık Rengi'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Dört Köşeyi de Yuvarlatma

Varsayılan olarak sol üst köşe karedir. Dört köşeyi de aynı şekilde yuvarlatmak için her iki yarıçap değişkenini ayarlayın:

[inline-code-attrs-start title = 'Dört Köşeyi de Yuvarlatma'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Kare Köşeler

[inline-code-attrs-start title = 'Kare Köşeler'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Markanıza Uygun Hale Getirme

[inline-code-attrs-start title = 'Marka Renkleri ve Köşeler'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Karanlık Modda Farklı Renkler

Widget karanlık moddayken `dark` sınıfına sahiptir, bu nedenle karanlık mod için farklı değerler ayarlayabilirsiniz:

[inline-code-attrs-start title = 'Karanlık Mod Kenarlık Renkleri'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Metin Alanı Neden Doğrudan Stillendirilmez?

Widget, yorum kutusu kenarlığının bir kısmını metin alanının etrafında kendisi çizer. `border-color` veya `border-radius` özelliklerini yalnızca `textarea` üzerinde ayarlarsanız, bu çizgiler varsayılan stilde kalır ve kenarlık uyumsuz görünür; örneğin yuvarlatılmış bir köşeden geçen kare bir çizgi gibi. Yukarıdaki değişkenler her ikisini de aynı anda değiştirir.