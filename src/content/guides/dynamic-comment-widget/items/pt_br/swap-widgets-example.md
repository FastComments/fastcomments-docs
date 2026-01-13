This is all done by configuring the [urlId](/guide-customizations-and-configuration.html#url-id) and related parameters and
calling `update()` on the widget instances to swap them out with a different thread.

The below is an interactive example (hit run in the top left) which allows you to add and swap comment threads on demand.
We could remove them as well by calling `instance.destroy` instead of `update`.

[inline-code-attrs-start title = 'Adicionar e Trocar Tópicos de Comentários'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
    <script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>

    <div id="fastcomments-widgets"></div>
    <div class="demo-form">
        <section>
            <h3>Add or Swap a Product/Comment Thread</h3>
            <label>
                Product Name
                <input type="text" name="name" />
            </label>
            <label>
                Product ID
                <input type="text" name="id" />
            </label>
            <label>
                Product URL
                <input type="url" name="url" />
            </label>
            <label>
                Replaces Product (id)
                <input type="text" name="replaces_id" />
            </label>
            <button type="button">Add/Swap</button>
        </section>
    </div>
    <script>
        (function () {
            const target = document.getElementById('fastcomments-widgets');
            const tenantId = "demo"; // COLOQUE SEU TENANT ID AQUI
    
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
                    "pageTitle": name, // para e-mails de notificação e painel de moderação
                    "url": productURL, // para e-mails de notificação e painel de moderação
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
                alert(`Product ${oldId} not found to replace!`);
            }
    
            // add some widgets
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

This is similar to how pagination works.

With the React and related libraries we can simply update the configuration and the library (React, Angular, Vue)
will handle refreshing the widget for us. With VanillaJS we have to manage it ourselves as shown.

Feel free to ask any questions below and we'll do our best to help!