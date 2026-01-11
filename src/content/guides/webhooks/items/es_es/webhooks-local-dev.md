Para el desarrollo local, usa una herramienta como [ngrok](https://ngrok.com/).

Para simplificar el mantenimiento de la seguridad del sistema, el desarrollo local sigue el mismo proceso que la configuración y protección de otros entornos. 

### Paso 1: Añade "localhost" a los dominios de tu cuenta.

Añade "localhost" [como dominio aquí](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Paso 2: Elige una API Key

Vamos a añadir la configuración de webhook para tu dominio, así que necesitaremos una API key. [Puedes hacerlo aquí.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

En "Associate with domain" - selecciona tu dominio "localhost".

**NOTA: Alternativamente, puedes usar un único API Secret para toda la actividad de pruebas y entornos de staging. Simplemente añade un API Secret para "All Domains", y ponle un nombre como "test".**

Asegúrate de tener definido un API Secret para tu(s) dominio(s) de producción. Los eventos de todos los demás dominios usarán el secret comodín (de pruebas).

### Paso 3: Añade tu Webhook

Mientras estés ejecutando ngrok u otra herramienta similar, establece el valor para "localhost" [aquí](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

Al hacer clic en `Send Test Payload`, enviaremos dos eventos de prueba para comprobar que validas la API key.

Una vez que lo valide, pulsa `Save`.

### Paso 4: Añade un comentario

Ahora puedes añadir, editar o eliminar comentarios y deberías ver que llamamos a tu máquina de desarrollo local con los eventos, usando tu API key de pruebas. Puede haber un retraso de hasta 30 segundos
para que los eventos lleguen a tu máquina.