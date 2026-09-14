Cuatro vals públicos que puedes remezclar, cada uno cubriendo una parte de esta guía.

**[Blog con comentarios](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) es un blog en Markdown con un hilo bajo cada publicación y recuentos de comentarios en bloque en el índice. Funciona en el momento en que lo remezclas, y una variable de entorno lo apunta a tu propia cuenta.

**[Demo SSO](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) inicia sesión al visitante con su cuenta de Val Town y entrega esa identidad al widget, por lo que no hay un segundo inicio de sesión.

**[Receptor de Webhook](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) verifica la firma HMAC en cada entrega y almacena los eventos en SQLite. Tiene un botón que firma una carga de prueba y la entrega a sí mismo, para que puedas observar la verificación exitosa antes de configurar un webhook real.

**[Habilidades de agente](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) es una biblioteca de habilidades de agente de FastComments que cubren el widget, SSO, la API REST, la moderación y la migración desde Disqus. Remezcla esto y el agente de Val Town, Townie, recoge automáticamente las habilidades de `skills/`, de modo que tu agente sepa cómo integrar los comentarios sin que tengas que pegar la documentación en el chat.

Las mismas habilidades se instalan en cualquier otro lugar con `npx skills add fastcomments/skills`.