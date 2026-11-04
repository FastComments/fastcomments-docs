Це все робиться шляхом налаштування [urlId](/guide-customizations-and-configuration.html#url-id) та пов'язаних параметрів і
виклику `update()` на екземплярах віджетів, щоб замінити їх на інший потік коментарів.

Нижче наведено інтерактивний приклад (натисніть run у верхньому лівому куті), який дозволяє додавати та замінювати потоки коментарів на вимогу.
Ми також могли б видалити їх, викликавши `instance.destroy` замість update.

[inline-code-attrs-start title = 'Додавання та заміна потоків коментарів'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
    <script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>

    <div id="fastcomments-widgets"></div>
    <div class="demo-form">
        <section>
            <h3>Додати або замінити продукт/потік коментарів</h3>
            <label>
                Назва продукту
                <input type="text" name="name" />
            </label>
            <label>
                ID продукту
                <input type="text" name="id" />
            </label>
            <label>
                URL продукту
                <input type="url" name="url" />
            </label>
            <label>
                Замінює продукт (id)
                <input type="text" name="replaces_id" />
            </label>
            <button type="button">Додати/Замінити</button>
        </section>
    </div>
    <script>
        (function () {
            const target = document.getElementById('fastcomments-widgets');
            const tenantId = "demo"; // ВСТАВТЕ СВІЙ TENANT ID ТУТ
    
            function setHeading(heading, {name, productID}) {
                heading.innerHTML = `${name} (${productID})`;
            }
    
            function addWidget({name, productID, productURL}) {
                const outer = document.createElement('div');
                const heading = document.createElement('h5');
                setHeading(heading, {name, productID});
                const inner = document.createElement('div');
                inner.append(heading);
                outer.append(inner);
                const widgetTarget = document.createElement('div');
                outer.append(widgetTarget);
                target.append(outer);
                window.FastCommentsUI(widgetTarget, {
                    "tenantId": tenantId,
                    "pageTitle": name, // для листів зі сповіщеннями та панелі модерації
                    "url": productURL, // для листів зі сповіщеннями та панелі модерації
                    "urlId": productID,
                    "showLiveRightAway": true
                });
            }
    
            function switchProduct(oldId, {name, productID, productURL}) {
                for (const instanceWrapped of fcUIInstances) {
                    if (instanceWrapped.targetElement) {
                        const config = instanceWrapped.config;
                        if (config.urlId === oldId) {
                            const heading = instanceWrapped.targetElement.parentNode.querySelector('h5');
                            setHeading(heading, {name, productID});
                            config.pageTitle = name;
                            config.urlId = productID;
                            config.url = productURL;
                            instanceWrapped.instance.update(config);
                            return;
                        }
                    }
                }
                alert(`Продукт ${oldId} для заміни не знайдено!`);
            }
    
            // додамо кілька віджетів
            addWidget({name: "Product A", productID: 'product-id-a', productURL: 'https://example.com/a'});
            addWidget({name: "Product B", productID: 'product-id-b', productURL: 'https://example.com/b'});
    
            const form = document.querySelector('.demo-form');
            form.querySelector('button').addEventListener('click', function() {
                const name = form.querySelector('input[name=name]').value;
                const productID = form.querySelector('input[name=id]').value;
                const productURL = form.querySelector('input[name=url]').value;
                const replaces_id = form.querySelector('input[name=replaces_id]').value;
    
                if (replaces_id) {
                    switchProduct(replaces_id, {
                        name,
                        productID,
                        productURL,
                    });
                } else {
                    addWidget({name, productID, productURL});
                }
            });
        })();
    </script>
    <style>
        #fastcomments-widgets {
            display: flex;
        }
        #fastcomments-widgets > div {
            width: 100%;
        }
    
        .demo-form section label {
            display: block;
            margin-bottom: 20px;
        }
    </style>
[inline-code-end]

Це схоже на те, як працює пагінація.

З React та пов'язаними бібліотеками ми можемо просто оновити конфігурацію, і бібліотека (React, Angular, Vue)
сама подбає про оновлення віджета. З VanillaJS нам доводиться керувати цим самостійно, як показано вище.

Не соромтеся ставити будь-які запитання нижче, і ми зробимо все можливе, щоб допомогти!