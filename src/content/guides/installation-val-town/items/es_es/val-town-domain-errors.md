Una vez que desactives el inquilino `demo`, el widget puede negarse a cargarse con un error de autorización. Esto se debe a que FastComments no sabe que debe permitir que tu cuenta se use en ese dominio.

[Ve aquí para agregar tu sitio a tu cuenta.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town merece una segunda mirada aquí, porque un val puede ser accesible en más de un nombre de host:

- Cada val HTTP tiene un endpoint predeterminado largo, `<org>--<id>.web.val.run`.
- Reclamar un subdominio personalizado agrega `<name>.val.run`.
- Un [dominio personalizado](https://docs.val.town/vals/http/custom-domains/) agrega un tercero.
- Las ramas obtienen sus propias URLs.

Agrega los nombres de host desde los que realmente sirvas el widget. Si reclamas un subdominio después de configurar todo, agrégalo también, de lo contrario el widget funcionará en la URL antigua y fallará en la nueva.