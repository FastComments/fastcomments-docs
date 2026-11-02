Cada sitio de WordPress necesita su propia cuenta o inquilino de FastComments. WordPress numera las publicaciones por separado en cada sitio, por lo que la publicación 5 en un sitio y la publicación 5 en otro compartirían un mismo hilo de comentarios si ambos estuvieran conectados a la misma cuenta. Por esa razón, la configuración del plugin se niega a conectar un sitio cuando la cuenta ya tiene otro diferente conectado.

[Puedes ver tus integraciones activas aquí.](https://fastcomments.com/auth/my-account/manage-data/integrations)

Para ejecutar varios sitios de WordPress, crea un [inquilino](https://fastcomments.com/auth/my-account/tenants) para cada uno, cámbiate a él y luego conecta el sitio desde la página de configuración del plugin. Los inquilinos están incluidos en los planes con marca blanca. También puedes crear una cuenta de FastComments separada para cada sitio, usando una dirección de correo electrónico diferente para cada uno.

Si un sitio se ha trasladado, por ejemplo de un entorno de pruebas a producción, desactiva el plugin en el sitio antiguo y elimina el sitio antiguo de tus integraciones activas. Luego conecta el nuevo. Los comentarios ya sincronizados del sitio antiguo permanecen en la cuenta y se muestran en cualquier publicación con el mismo ID en el nuevo sitio.

Puedes usar la misma cuenta de FastComments con una única instalación de WordPress que abarque varios dominios y tenga el mismo contenido en esos dominios.