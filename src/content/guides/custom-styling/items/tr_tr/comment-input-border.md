The border around the comment box is made of the text area's own border plus a few thin lines the widget draws around it. To change its color or corner roundness, set these CSS variables instead of styling the `textarea` directly. The variables restyle every part of the border at once, so the corners and colors always line up.

| Değişken | Ne değiştirir | Varsayılan |
|---|---|---|
| `--fc-input-border-color` | Kenarlık rengi | `#bfbfbf` |
| `--fc-input-border-color-focus` | Kullanıcı yazarken kenarlık rengi | `#555` |
| `--fc-input-border-radius` | Yuvarlatılmış köşelerin yuvarlaklığı | `11px` |
| `--fc-input-border-start-start-radius` | Kare üst sol köşenin yuvarlaklığı (sağdan sola dillerde üst sağ) | `0` |

Add the CSS to the **Custom CSS** box on the [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), or pass it with the `customCSS` option. You only need to set the variables you want to change.

You can also use the **Comment box border** helper right under the Custom CSS box, which writes this CSS for you.

## Kenarlık Rengini Değiştir

[inline-code-attrs-start title = 'Kenarlık Rengi'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Yazarken Kenarlık Rengini Değiştir

[inline-code-attrs-start title = 'Yazarken Kenarlık Rengi'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Tüm Dört Köşeyi Yuvarlat

Varsayılan olarak üst sol köşe karedir. Tüm dört köşeyi aynı şekilde yuvarlamak için her iki yarıçap değişkenini de ayarlayın:

[inline-code-attrs-start title = 'Tüm Dört Köşeyi Yuvarlat'; type = 'css'; isFunctional = false; inline-code-attrs-end]
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

## Markanıza Uygun Hale Getirin

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

Widget karanlık modda olduğunda `dark` sınıfına sahiptir, bu yüzden karanlık mod için farklı değerler ayarlayabilirsiniz:

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

## Neden Metin Alanını Doğrudan Stilize Etmiyoruz?

The widget draws part of the comment box border itself, around the text area. If you set `border-color` or `border-radius` only on the `textarea`, those lines keep the default style and the border looks mismatched, for example a square line running through a rounded corner. The variables above change both at once.