FastComments'i Memberstack ile küçük bir kod parçacığı ile kolayca bağlayabiliriz:

[inline-code-attrs-start title = 'FastComments Memberstack Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.$memberstackDom.getCurrentMember().then(({data: member}) => {
        if (member) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: {
                    username: member.customFields.name || member.auth.email.replace(/@.+/, ''),
                    email: member.auth.email,
                    avatar: member.customFields.avatar
                }
            });
        } else {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: null
            });
        }
    })
</script>
[inline-code-end]

Kullanıcı Memberstack üzerinden giriş yapmışken sitenizi veya uygulamanızı ziyaret ettiğinde, otomatik olarak FastComments'e giriş yapacak ve yorumları
`Verified` olarak işaretlenecektir.

**Giriş yapmamışlarsa, adlarını ve e-postalarını bırakarak da yorum yapabilecekler.**

### Sadece Üyelere Özel Yorum

**Sadece üyelere özel yorum** istiyorsanız, aşağıdaki kod parçacığını deneyin, ancak `https://example.com/login` adresini kullanıcıların `Login` düğmesine tıkladıklarında gitmelerini istediğiniz yere değiştirin:

[inline-code-attrs-start title = 'Exclusive FastComments Memberstack Snippet'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.$memberstackDom.getCurrentMember().then(({data: member}) => {
        if (member) {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: {
                    username: member.customFields.name || member.auth.email.replace(/@.+/, ''),
                    email: member.auth.email,
                    avatar: member.customFields.avatar
                }
            });
        } else {
            FastCommentsUI(document.getElementById('fastcomments-widget'), {
                tenantId: "demo",
                urlId: window.location.pathname,
                simpleSSO: {
                    loginURL: 'https://example.com/login'
                }
            });
        }
    })
</script>
[inline-code-end]
