Това се прави чрез конфигуриране на [urlId](/guide-customizations-and-configuration.html#url-id) и свързаните параметри и
извикване на `update()` върху инстанциите на уиджетите, за да ги замените с различна нишка.

Долу е интерактивен пример (натиснете "Run" в горния ляв ъгъл), който ви позволява да добавяте и сменяте нишки с коментари при поискване.
Можем също да ги премахнем, като извикаме `instance.destroy` вместо `update`.

[inline-code-attrs-start title = 'Добавяне и смяна на нишки за коментари'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            const tenantId = "demo"; // ПОСТАВЕТЕ ВАШИЯ tenantId ТУК
    
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
                    "pageTitle": name, // за имейли за уведомления и панела за модериране
                    "url": productURL, // за имейли за уведомления и панела за модериране
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
    
            // добавяме няколко уиджета
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

Това е подобно на начина, по който работи пагинацията.

С React и свързаните библиотеки можем просто да обновим конфигурацията и библиотеката (React, Angular, Vue)
ще се погрижи да опресни уиджета вместо нас. С VanillaJS трябва сами да го управляваме, както е показано.

Не се колебайте да задавате въпроси по-долу и ние ще направим всичко възможно, за да помогнем!