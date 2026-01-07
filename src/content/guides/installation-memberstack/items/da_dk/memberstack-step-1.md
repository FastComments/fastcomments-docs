Vi kan nemt forbinde FastComments med Memberstack med et lille kodestykke:

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

Når brugeren besøger dit websted eller din applikation, mens de er logget ind via Memberstack, vil de automatisk blive logget ind i FastComments, og deres kommentarer
vil blive markeret som `Verified`.

**De vil også kunne kommentere ved at efterlade deres navn og e-mail, hvis de ikke er logget ind.**

### Kun medlemskommentering

Hvis du gerne vil have **kun medlemskommentering**, prøv følgende kodestykke, men ændre `https://example.com/login` til det sted, hvor du vil have brugerne til at gå, når de klikker på `Login`-knappen:

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
