Para el desarrollo local, use una herramienta como [ngrok](https://ngrok.com/).

Para simplificar el mantenimiento de la seguridad del sistema, el desarrollo local sigue el mismo proceso que la configuración y protección de otros entornos.

### Paso 1: Agregar "localhost" a los dominios en su cuenta.

Agregue "localhost" [como un dominio aquí](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='El formulario de agregar dominio en la configuración de la cuenta con localhost ingresado en el campo de nombres de dominio'; title='Agregar localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Paso 2: Elegir una clave de API

Vamos a agregar la configuración de webhook para su dominio, por lo que necesitaremos una clave de API. [Puede hacerlo aquí.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; cacheBuster = 'v3'; selector = '.content .account-block'; alt='Formulario de nuevo secreto de API con el dominio asociado establecido en localhost y la clave llamada Testing'; title='Agregar clave de API de Testing'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

En "Asociar con dominio" - seleccione su dominio "localhost".

**NOTA: Alternativamente, puede usar un solo API Secret para toda la actividad de pruebas y los entornos de staging. Simplemente agregue un API Secret para "All Domains" y asígnele un nombre como "test".**

Asegúrese de tener un API Secret definido para su(s) dominio(s) de producción. Los eventos para todos los demás dominios usarán el secreto comodín (de pruebas).

### Paso 3: Agregar su webhook

Mientras ejecuta ngrok o una herramienta similar, establezca el valor para "localhost" [aquí](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; cacheBuster = 'v3'; selector = '.content'; alt='Administración de webhooks con el dominio localhost seleccionado y una URL de ngrok ingresada en el endpoint de comentario creado'; title='Agregar webhook de pruebas'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}]; app-screenshot-end]

Al hacer clic en `Send Test Payload`, enviaremos dos eventos de prueba para verificar que usted valide la clave de API.

Una vez validada, presione `Save`.

### Paso 4: Agregar un comentario

Ahora puede agregar, editar o eliminar comentarios y debería vernos llamar a su máquina de desarrollo local con los eventos, usando su clave de API de pruebas. Puede haber hasta 30 segundos de demora
para que los eventos lleguen a su máquina.