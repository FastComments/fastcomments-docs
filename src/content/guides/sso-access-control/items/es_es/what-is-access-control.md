Con el Control de Acceso SSO de FastComments, a veces denominado RBAC, los usuarios pueden estar restringidos para acceder únicamente a ciertas páginas o hilos de comentarios. Además, los usuarios solo pueden `@mention` entre ellos dentro del mismo grupo.

## En detalle

Podemos colocar `Users` y opcionalmente `Pages` en grupos.

Cuando `Users` se colocan en grupos, y `Limit Comments by SSO User Groups` está habilitado en la Configuración del widget, entonces los usuarios solo verán comentarios de usuarios en sus mismos grupos y solo podrán `@mention` a usuarios en esos mismos grupos.

Además, `Pages` pueden colocarse en grupos, y entonces los usuarios solo podrán acceder a comentarios de páginas a las que tengan acceso.

Llamamos a esto "grupos a nivel de usuario" frente a "grupos a nivel de página".

Entonces, ¿cuál es el adecuado para usted?

#### Utilice grupos a nivel de usuario si...

- Desea usar el mismo valor `urlId` (URL de la página, o ID del artículo), pero restringir los comentarios por grupo.
- Por ejemplo, desea tener los grupos "Usuario nuevo" y "Usuario veterano", y que nunca vean los comentarios del otro en las mismas páginas.

#### Utilice grupos a nivel de página si...

- Sus grupos tienen páginas específicas.
- Por ejemplo, los usuarios en el grupo "Páginas públicas" nunca deberían ver artículos en los artículos "Top Secret".