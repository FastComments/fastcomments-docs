Es común tener un sub tenant por entorno de test o dev con FastComments. Cada tenant tendría su propia configuración, datos y claves de API. La configuración, los datos y los usuarios no se pueden compartir entre tenants.
Todo está aislado. Sin embargo, los super admins del tenant padre pueden suplantar a usuarios en tenants hijos.

Hay dos enfoques:

- El tenant principal es para producción, y los sub-tenants son para entornos de test.
- El tenant principal es simplemente para facturación, y cada sub-tenant es para prod, test, etc.

El primero es generalmente más fácil de entender para los usuarios, pero esto puede depender de su organización.

Los tenants se pueden crear [aquí](https://eu.fastcomments.com/auth/my-account/tenants) si tiene el paquete. También es donde los super admins
suplantarían a los usuarios. Los tenants también se pueden crear a través de la API para configuraciones más personalizadas/automatizadas.

Independientemente del enfoque adoptado, tendrá que añadir los moderadores y usuarios que quieran ver los datos de producción en el tenant de "prod". Así, por ejemplo, si quiere
ir con la opción B y tener el tenant padre para facturación, y tener un sub tenant para "prod", querrá añadir el tenant, cambiar al nuevo tenant y añadir sus
usuarios admin y moderador para el sub-tenant.

Finalmente, para aclarar, la página de Moderate Comments estará vacía con la opción B para el tenant padre.