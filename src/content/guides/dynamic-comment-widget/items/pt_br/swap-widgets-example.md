Isso é feito configurando o [urlId](/guide-customizations-and-configuration.html#url-id) e os parâmetros relacionados e
chamando `update()` nas instâncias do widget para trocá-los por um thread diferente.

Abaixo está um exemplo interativo (clique em run no canto superior esquerdo) que permite adicionar e trocar threads de comentários sob demanda.
Também poderíamos removê-los chamando `instance.destroy` em vez de update.

[inline-code-attrs-start title = 'Adicionar e Trocar Threads de Comentários'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
    <script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>

    <div id="fastcomments-widgets"></div>
    <div class="demo-form">
        <section>
            <h3>Adicionar ou Trocar um Produto/Thread de Comentários</h3>
            <label>
                Nome do Produto
                <input type="text" name="name" />
            </label>
            <label>
                ID do Produto
                <input type="text" name="id" />
            </label>
            <label>
                URL do Produto
                <input type="url" name="url" />
            </label>
            <label>
                Substitui o Produto (id)
                <input type="text" name="replaces_id" />
            </label>
            <button type="button">Adicionar/Trocar</button>
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
                alert(`Produto ${oldId} não encontrado para substituir!`);
            }
    
            // adicionar alguns widgets
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

Isso é semelhante a como funciona a paginação.

Com o React e bibliotecas relacionadas, podemos simplesmente atualizar a configuração e a biblioteca (React, Angular, Vue)
cuidará de atualizar o widget para nós. Com VanillaJS temos que gerenciar isso nós mesmos, como mostrado.

Sinta-se à vontade para fazer qualquer pergunta abaixo e faremos o nosso melhor para ajudar!