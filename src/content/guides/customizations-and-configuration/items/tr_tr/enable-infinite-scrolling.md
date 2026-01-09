[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments widget'ı tüm görünür yorumlara uyacak şekilde dikey olarak yeniden boyutlanır. Sayfalama, mevcut sayfanın sonunda bulunan "Sonrakileri Görüntüle" düğmesiyle gerçekleştirilir; çoğu kullanıcı için bu etkileşimin en hoş hissettiren yöntem olduğunu gördük.

Ancak, sonsuz kaydırmanın tercih edildiği bazı durumlar vardır. Örneğin, bu özelliği Stream Chat ürünümüzde kullanıyoruz.

"Sonrakileri Görüntüle" düğmelerini gizleyip sonsuz kaydırmaya geçmek için **enableInfiniteScrolling** bayrağını true olarak ayarlayabiliriz:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Bu ayrıca özel CSS eklenmesini gerektirir. Kaydırmayı etkinleştirmek için `.comments` seçicisi için örneğin aşağıdaki gibi özel CSS ekleyin:

[inline-code-attrs-start title = 'Sonsuz Kaydırmayı Etkinleştir'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Tam çalışan bir örnek şöyle olacaktır:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

Yukarıdaki örnekte `customCSS` özelliğini kullanıyoruz; ancak performans nedenleriyle bunun yerine Widget Yapılandırma Arayüzü'nün kullanılması önerilir. [Özel CSS dokümantasyonuna bakın.](/guide-customizations-and-configuration.html#custom-css)