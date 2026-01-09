Para SSO, hay la siguiente configuración a considerar para las notificaciones:

- Si el usuario ha optado por recibir notificaciones.
  - Esto se realiza estableciendo el indicador `optedInNotifications` en `true` o `false` en el objeto `SSOUser`.
  - Esto puede configurarse a través de la API.
  - Además, si envías un valor para este indicador en la carga útil, se actualizará automáticamente cuando el usuario cargue un hilo de comentarios.
- Si el usuario ha optado por las notificaciones de **suscripción**.
  - Esto se realiza estableciendo el indicador `optedInSubscriptionNotifications` en `true` o `false` en el objeto `SSOUser`.
  - Esto puede configurarse a través de la API.
  - Además, si envías un valor para este indicador en la carga útil, se actualizará automáticamente cuando el usuario cargue un hilo de comentarios.
- Definir su dirección de correo electrónico.
  - Si no está presente, no podemos enviar notificaciones por correo electrónico.
- Si desactivar los enlaces para darse de baja en los correos electrónicos.
  - Esto se hace mediante el indicador `disableUnsubscribeLinks` en el objeto `Tenant`.
  - Esto puede configurarse a través de la API.
- Si usar un enlace personalizado para darse de baja.
  - Esto se hace mediante la propiedad `footerUnsubscribeURL` en el objeto `DomainConfig`.
  - Esto puede configurarse a través de la API.
  - También puede considerar establecer los encabezados de cancelación de suscripción relevantes mediante `emailHeaders` en el mismo objeto.