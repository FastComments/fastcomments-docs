Vse to se izvede z nastavitvijo [urlId](/guide-customizations-and-configuration.html#url-id) in sorodnih parametrov ter
klicem `update()` na primerkih widgetov, da jih zamenjate z drugo nitjo.

Spodaj je interaktivni primer (pritisnite Run v zgornjem levem kotu), ki vam omogoča, da po potrebi dodate in zamenjate niti komentarjev.
Lahko jih tudi odstranimo z klicem `instance.destroy` namesto `update`.

[inline-code-attrs-start title = 'Dodaj in zamenjaj niti komentarjev'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            const tenantId = "demo"; // VSTAVITE TUKAJ SVOJ tenant ID
    
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
                    "pageTitle": name, // za e-poštna obvestila in nadzorno ploščo moderiranja
                    "url": productURL, // za e-poštna obvestila in nadzorno ploščo moderiranja
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
    
            // dodaj nekaj widgetov
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

To je podobno, kot deluje straničenje.

S knjižnicami React in sorodnimi lahko preprosto posodobimo konfiguracijo, knjižnica (React, Angular, Vue)
bo poskrbela za osvežitev widgeta za nas. Pri VanillaJS pa ga moramo upravljati sami, kot je prikazano.

Prosto postavite kakršna koli vprašanja spodaj in potrudili se bomo pomagati!