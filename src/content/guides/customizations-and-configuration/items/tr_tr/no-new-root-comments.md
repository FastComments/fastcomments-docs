[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

`noNewRootComments` öğesini `true` olarak ayarlamak widget'ın kök yanıt alanını gizlemesine neden olur, ancak kullanıcıların yanıt
vermelerine yine de izin verir. Örneğin, yalnızca bazı kullanıcıların üst düzey yorum bırakmasına izin vermek için bunu sayfa yüklenirken koşullu olarak ayarlayabilirsiniz.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]