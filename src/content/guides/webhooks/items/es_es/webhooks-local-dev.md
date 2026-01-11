Para el desarrollo local, use una herramienta como [ngrok](https://ngrok.com/).

Para simplificar el mantenimiento de la seguridad del sistema, el desarrollo local sigue el mismo proceso que la configuración y aseguramiento de otros entornos. 

### Paso 1: Añadir "localhost" a los dominios en su cuenta.

Añada "localhost" [como dominio aquí](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Paso 2: Seleccione una API Key

Vamos a añadir la configuración del webhook para su dominio, así que necesitaremos una API key. [Puede hacerlo aquí.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Bajo "Associate with domain" - seleccione su dominio "localhost".

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Asegúrese de tener definido un API Secret para su(s) dominio(s) de producción. Los eventos de todos los demás dominios usarán el secreto comodín (testing).

### Paso 3: Añada su Webhook

Mientras ejecuta ngrok u otra herramienta similar, establezca el valor para "localhost" [aquí](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Al hacer clic en `Send Test Payload`, enviaremos dos eventos de prueba para verificar que valida la API key.

Una vez que lo valide, pulse `Save`.

### Paso 4: Añada un comentario

Ahora puede añadir, editar o eliminar comentarios y debería ver que llamamos a su máquina de desarrollo local con los eventos, usando su testing API key. Puede haber un retraso de hasta 30 segundos para que los eventos lleguen a su máquina.