이 모든 것은 [urlId](/guide-customizations-and-configuration.html#url-id) 및 관련 매개변수를 구성하고 위젯 인스턴스에서 `update()`를 호출하여 다른 스레드로 교체함으로써 수행됩니다.

아래는 대화형 예제입니다(왼쪽 상단의 실행 버튼을 누르세요). 이 예제는 필요에 따라 댓글 스레드를 추가하고 교체할 수 있게 해줍니다. 또한 `instance.destroy`를 호출하여 update 대신 제거할 수도 있습니다.

[inline-code-attrs-start title = '댓글 스레드 추가 및 교체'; type = 'html'; isFunctional = true; inline-code-attrs-end]
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
            const tenantId = "demo"; // 여기에 귀하의 테넌트 ID를 넣으세요
    
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
                    "pageTitle": name, // 알림 이메일 및 검토 대시보드를 위한
                    "url": productURL, // 알림 이메일 및 검토 대시보드를 위한
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

이것은 페이지네이션이 작동하는 방식과 유사합니다.

React 및 관련 라이브러리를 사용할 경우 구성(configuration)을 업데이트하면 라이브러리(React, Angular, Vue)가 위젯 새로고침을 처리해 줍니다. VanillaJS에서는 아래 예제처럼 직접 관리해야 합니다.

아래에 질문을 남기시면 최선을 다해 도와드리겠습니다!