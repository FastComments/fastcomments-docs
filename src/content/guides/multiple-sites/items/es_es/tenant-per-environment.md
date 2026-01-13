Es común tener un sub tenant por entorno de prueba o desarrollo con FastComments. Cada tenant tendría su propia configuración, datos, y claves de API. La configuración, los datos y los usuarios no pueden compartirse entre tenants.
Todo está aislado. Sin embargo, los superadmins del tenant principal pueden suplantar a usuarios en tenants secundarios.

There are two approaches:

- El tenant principal es para producción, y los sub-tenants son para entornos de prueba.
- El tenant principal es simplemente para facturación, y cada sub-tenant es para prod, test, y así sucesivamente.

La primera suele ser, por lo general, más fácil de comprender para los usuarios, pero esto puede depender de tu organización.

Tenants can be created [aquí](https://eu.fastcomments.com/auth/my-account/tenants) if you have the package. This is also where super admins would
suplantar a usuarios. Los tenants también pueden crearse mediante la API para configuraciones más personalizadas/automatizadas.

Sea cual sea el enfoque elegido, tendrás que añadir a los moderadores y usuarios que quieran ver los datos de producción en el tenant "prod". Por ejemplo, si quieres
optar por la opción B y usar el tenant principal para facturación, y tener un sub tenant para "prod", querrás añadir el tenant, cambiar al nuevo tenant, y añadir tus
usuarios administradores y moderadores para el sub-tenant. 

Finalmente, para aclarar, la página Moderar comentarios estará vacía con la opción B para el tenant principal.