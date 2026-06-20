### PyPI

```bash
pip install fastcomments
```

### Contenido de la biblioteca

Esta biblioteca contiene dos módulos: el cliente de API generado y la biblioteca central de Python, que incluye utilidades escritas a mano para facilitar el trabajo con la API, incluido el soporte SSO.

- [Documentación de la biblioteca del cliente de API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentación de la biblioteca central, incluidos ejemplos de SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### APIs públicas vs APIs protegidas

Para el cliente de API, hay tres clases, `DefaultApi`, `PublicApi`, y `ModerationApi`. El `DefaultApi` contiene métodos que requieren tu clave de API, y `PublicApi` contiene métodos que pueden realizarse directamente desde un navegador/dispositivo móvil/etc. sin autenticación. El `ModerationApi` impulsa el panel de moderador y contiene métodos para moderar comentarios (listar, contar, buscar, registros, exportar), acciones de moderación (eliminar/restaurar, marcar, establecer estado de revisión/spam/aprobación, votos, reabrir/cerrar hilo), bloqueos (bloquear para comentar, deshacer, resúmenes previos al bloqueo, estado/preferencias de bloqueo, recuentos de usuarios bloqueados), e insignias y confianza (otorgar/eliminar insignia, insignias manuales, obtener/establecer factor de confianza, perfil interno del usuario). Cada método de `ModerationApi` acepta un parámetro `sso` para que pueda ser invocado en nombre de un moderador autenticado mediante SSO.