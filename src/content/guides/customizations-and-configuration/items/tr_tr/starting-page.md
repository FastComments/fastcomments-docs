---
[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Yorumları getirirken ve render ederken, yorum bileşeninin hangi sayfadan başlayacağını bilmesi gerekir. Varsayılan olarak, o
ilk sayfadan başlar, yalnızca o sayfayı render eder.

İstenirse, render edilecek tam sayfa, *startingPage* ayarı olarak yorum bileşenine geçirilebilir.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Sayfa numaralarının sıfırdan başladığını unutmayın, bu yüzden yukarıdaki örnek ikinci sayfayı render eder.

---