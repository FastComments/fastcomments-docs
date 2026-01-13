`Extension` ile etkileşim basittir; çağrılmasını istediğimiz fonksiyonlara referanslar tanımlamak yeterlidir.

Önceki örnekten yola çıkarak, her yorumun üstüne HTML eklemek istediğimizi varsayalım:

[inline-code-attrs-start title = 'Basit Bir Eklenti - Devam'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    extension.commentFilter = function(comment, html) {
        return `<h3>The user's name is ${comment.commenterName}!</h3>` + html;
    }
})();
[inline-code-end]

Böyle HTML döndürdüğünüzde, dom-diffing algoritması aracılığıyla UI'ya birleştirilir.

#### Bir yorumun yeniden render edilmesini elle tetikleme

İlk sayfa yüklemesini bekleyip `reRenderComment` çağırarak bir yorumu elle yeniden render edebiliriz:

[inline-code-attrs-start title = 'Bir Yorumu Yeniden Render Etme'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });

    let renderCount = 0;

    extension.commentFilter = function(comment, html) {
        renderCount++;
        return `<h3>The render count is ${renderCount}!</h3>` + html;
    }

    extension.onInitialRenderComplete = function() {
        setInterval(function() {
            extension.reRenderComment(extension.commentsById[Object.keys(extension.commentsById)[0]], function renderDone() {
                console.log('Comment re-render done.');
            });
        }, 2000); // zaman aşımı gerekli değil, sadece bir örnek.
    }
})();
[inline-code-end]

---