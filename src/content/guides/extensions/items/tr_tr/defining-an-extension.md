---
Olabilecek en küçük uzantı şöyle olur:

[inline-code-attrs-start title = 'Basit Bir Uzantı'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Bu örnek için bunu `my-extension.js` olarak kaydedin ve `https://example.com/my-extension.min.js` adresinde erişilebilir hale getirin.

Bu uzantı herhangi bir şey yapmaz; yüklenince çekirdek yorum kütüphanesi tarafından oluşturulan uzantı nesnesini alır.

Bu `Extension` nesnesi tekil (singleton) bir nesnedir ve diğer uzantılarla paylaşılmaz.

Sonraki adım, uzantımızı yüklemek için yorum widget'ına bunu bildirmektir. Örneğin:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

İşlevsel örnekler için bir sonraki bölüme bakın.

---