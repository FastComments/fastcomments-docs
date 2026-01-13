[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Varsayılan olarak, FastComments yorum bileşeni çoğu sitede koyu modu otomatik olarak algılar.

Koyu mod algılandığında, FastComments beyaz arka plan üzerindeki siyah metinden siyah arka plan üzerindeki beyaz metne geçer. Görseller de değişir.

Sayfa yüklendiğinde, bileşen yorum bileşeninin arkasındaki sayfa arka planının ne kadar koyu olduğunu belirlemeye çalışır. Bu, sayfanın beyaz bir arka plana sahip olabileceği anlamına gelir; ancak yorum bileşenini siyah arka plana sahip bir kapsayıcı içine koyarsanız, yorumların okunabilir olması için koyu mod yine de otomatik olarak etkinleştirilmelidir.

Bununla birlikte, "luminance" belirlemeye dayanan algılama mekanizması istediğiniz zaman koyu modu etkinleştirmeyebilir. Zorla etkinleştirmek için *hasDarkBackground* bayrağını aşağıdaki gibi true olarak ayarlayın:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---