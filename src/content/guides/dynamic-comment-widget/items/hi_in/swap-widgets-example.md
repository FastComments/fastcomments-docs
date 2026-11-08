यह सब [urlId](/guide-customizations-and-configuration.html#url-id) और संबंधित पैरामीटर को कॉन्फ़िगर करके तथा विजेट इंस्टेंसेस पर `update()` को कॉल करके उन्हें किसी अलग थ्रेड से बदलकर किया जाता है।

नीचे एक इंटरैक्टिव उदाहरण दिया गया है (ऊपर बाईं ओर run दबाएं) जो आपको मांग पर कमेंट थ्रेड जोड़ने और बदलने की सुविधा देता है।
हम अपडेट के बजाय `instance.destroy` को कॉल करके उन्हें हटा भी सकते हैं।

[inline-code-attrs-start title = 'कमेंट थ्रेड जोड़ें और बदलें'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
    <script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>

    <div id="fastcomments-widgets"></div>
    <div class="demo-form">
        <section>
            <h3>प्रोडक्ट/कमेंट थ्रेड जोड़ें या बदलें</h3>
            <label>
                प्रोडक्ट का नाम
                <input type="text" name="name" />
            </label>
            <label>
                प्रोडक्ट ID
                <input type="text" name="id" />
            </label>
            <label>
                प्रोडक्ट URL
                <input type="url" name="url" />
            </label>
            <label>
                बदला जाने वाला प्रोडक्ट (id)
                <input type="text" name="replaces_id" />
            </label>
            <button type="button">जोड़ें/बदलें</button>
        </section>
    </div>
    <script>
        (function () {
            const target = document.getElementById('fastcomments-widgets');
            const tenantId = "demo"; // PUT YOUR TENANT ID HERE
    
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
                    "pageTitle": name, // for notification emails and moderation dashboard
                    "url": productURL, // for notification emails and moderation dashboard
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

यह उसी तरह काम करता है जैसे पेजिनेशन काम करता है।

React और संबंधित लाइब्रेरीज़ के साथ हम बस कॉन्फ़िगरेशन अपडेट कर सकते हैं और लाइब्रेरी (React, Angular, Vue)
हमारे लिए विजेट को रिफ्रेश करने का काम संभाल लेगी। VanillaJS के साथ हमें इसे स्वयं प्रबंधित करना पड़ता है, जैसा ऊपर दिखाया गया है।

नीचे कोई भी प्रश्न पूछने में संकोच न करें और हम मदद करने की पूरी कोशिश करेंगे!